//! linux-df-parser
//! ===============
//!
//! A simple parser for the Linux `df` command. To get numbers in bytes, call `df` with `-B1`
//! argument: `/bin/df -B1`
//!
//! Usage
//! -----
//! ```
//! # use linux_df_parser::Df;
//! let s = r#"
//!     df: /run/user/1000/doc: Operation not permitted
//!     Filesystem                 1B-blocks         Used    Available Use% Mounted on
//!     udev                     12294803456            0  12294803456   0% /dev
//!     /dev/nvme0n1p2             493201408    121312256    346304512  26% /boot
//! "#.trim();
//! let df = Df::from(s);
//! assert_eq!(df.get_by_filesystem("/dev/nvme0n1p2").unwrap().used, 121312256);
//! ```

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

/// `df` command representation
#[derive(Debug)]
pub struct Df(pub Vec<DfLine>);

/// A line of the `df` command
#[derive(Debug)]
pub struct DfLine {
    /// Filesystem
    pub filesystem: String,
    /// Mount point
    pub mounted: String,
    /// Total size
    pub total: u64,
    /// Used size
    pub used: u64,
}

impl From<&str> for Df {
    fn from(value: &str) -> Self {
        Self(value.lines().filter_map(DfLine::from_str).collect())
    }
}

impl Df {
    /// Returns a [`DfLine`] by filesystem
    pub fn get_by_filesystem(&self, filesystem: &str) -> Option<&DfLine> {
        self.0.iter().find(|x| x.filesystem == filesystem)
    }

    /// Returns a [`DfLine`] by mount point
    pub fn get_by_mount(&self, mounted: &str) -> Option<&DfLine> {
        self.0.iter().find(|x| x.mounted == mounted)
    }
}

impl DfLine {
    fn from_str(s: &str) -> Option<Self> {
        let mut parts = s.split_whitespace();
        let filesystem = parts.next()?.into();
        let total = parts.next()?.parse().ok()?;
        let used = parts.next()?.parse().ok()?;
        parts.next()?;
        parts.next()?;
        let mounted = parts.next()?.into();
        Some(Self {
            filesystem,
            mounted,
            total,
            used,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_from_str() {
        assert!(DfLine::from_str("df: /run/user/1000/doc: Operation not permitted").is_none());
        assert!(DfLine::from_str(
            "Filesystem                 1B-blocks         Used    Available Use% Mounted on"
        )
        .is_none());

        let line = DfLine::from_str(
            "udev                     12294803456            0  12294803456   0% /dev",
        )
        .unwrap();
        assert_eq!(line.filesystem, "udev");
        assert_eq!(line.mounted, "/dev");
        assert_eq!(line.total, 12294803456);
        assert_eq!(line.used, 0);
    }

    #[test]
    fn test_df() {
        let src = r#"
            df: /run/user/1000/doc: Operation not permitted
            Filesystem                 1B-blocks         Used    Available Use% Mounted on
            udev                     12294803456            0  12294803456   0% /dev
            /dev/nvme0n1p2             493201408    121312256    346304512  26% /boot
            /dev/nvme0n1p1             535805952      3579904    532226048   1% /boot/efi
        "#
        .trim();

        // From str
        let df = Df::from(src);
        assert_eq!(df.0.len(), 3);
        assert_eq!(df.0.get(1).unwrap().mounted, "/boot");

        // `get_by_filesystem`
        assert!(df.get_by_filesystem("unknown").is_none());
        assert_eq!(df.get_by_filesystem("udev").unwrap().mounted, "/dev");

        // `get_by_mount`
        assert!(df.get_by_mount("unknown").is_none());
        assert_eq!(df.get_by_mount("/dev").unwrap().filesystem, "udev");
    }
}

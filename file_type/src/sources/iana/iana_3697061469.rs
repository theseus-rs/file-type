use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3697061469: FileType = FileType {
    file_format: &FileFormat {
        id: 3_697_061_469,
        source_type: SourceType::Iana,
        name: "vnd.gentoo.xpak",
        extensions: &[],
        media_types: &["application/vnd.gentoo.xpak"],
        signatures: &[],
        related_formats: &[],
    },
};

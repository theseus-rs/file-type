use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1468214095: FileType = FileType {
    file_format: &FileFormat {
        id: 1_468_214_095,
        source_type: SourceType::Iana,
        name: "vnd.gentoo.pkgmetadata+xml",
        extensions: &[],
        media_types: &["application/vnd.gentoo.pkgmetadata+xml"],
        signatures: &[],
        related_formats: &[],
    },
};

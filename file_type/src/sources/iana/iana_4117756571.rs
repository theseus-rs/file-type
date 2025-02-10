use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4117756571: FileType = FileType {
    file_format: &FileFormat {
        id: 4_117_756_571,
        source_type: SourceType::Iana,
        name: "vnd.gentoo.manifest",
        extensions: &[],
        media_types: &["application/vnd.gentoo.manifest"],
        signatures: &[],
        related_formats: &[],
    },
};

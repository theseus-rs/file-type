use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1474921673: FileType = FileType {
    file_format: &FileFormat {
        id: 1_474_921_673,
        source_type: SourceType::Iana,
        name: "vnd.omads-folder+xml",
        extensions: &[],
        media_types: &["application/vnd.omads-folder+xml"],
        signatures: &[],
        related_formats: &[],
    },
};

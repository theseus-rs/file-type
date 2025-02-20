use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4260785652: FileType = FileType {
    file_format: &FileFormat {
        id: 4_260_785_652,
        source_type: SourceType::Iana,
        name: "vnd.cryptii.pipe+json",
        extensions: &[],
        media_types: &["application/vnd.cryptii.pipe+json"],
        signatures: &[],
        related_formats: &[],
    },
};

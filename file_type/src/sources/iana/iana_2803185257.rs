use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2803185257: FileType = FileType {
    file_format: &FileFormat {
        id: 2_803_185_257,
        source_type: SourceType::Iana,
        name: "cdmi-queue",
        extensions: &[],
        media_types: &["application/cdmi-queue"],
        signatures: &[],
        related_formats: &[],
    },
};

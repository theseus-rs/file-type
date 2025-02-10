use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1142814227: FileType = FileType {
    file_format: &FileFormat {
        id: 1_142_814_227,
        source_type: SourceType::Iana,
        name: "vnd.crick.clicker.palette",
        extensions: &[],
        media_types: &["application/vnd.crick.clicker.palette"],
        signatures: &[],
        related_formats: &[],
    },
};

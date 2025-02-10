use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_730164878: FileType = FileType {
    file_format: &FileFormat {
        id: 730_164_878,
        source_type: SourceType::Iana,
        name: "vnd.amazon.mobi8-ebook",
        extensions: &[],
        media_types: &["application/vnd.amazon.mobi8-ebook"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_851476558: FileType = FileType {
    file_format: &FileFormat {
        id: 851_476_558,
        source_type: SourceType::Linguist,
        name: "vCard",
        extensions: &["vcf"],
        media_types: &["text/x-properties"],
        signatures: &[],
        related_formats: &[],
    },
};

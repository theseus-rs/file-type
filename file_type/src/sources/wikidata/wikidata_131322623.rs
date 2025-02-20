use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131322623: FileType = FileType {
    file_format: &FileFormat {
        id: 131_322_623,
        source_type: SourceType::Wikidata,
        name: "TSX",
        extensions: &["tsx"],
        media_types: &["text/typescript-tsx"],
        signatures: &[],
        related_formats: &[],
    },
};

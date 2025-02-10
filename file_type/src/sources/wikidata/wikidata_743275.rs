use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_743275: FileType = FileType {
    file_format: &FileFormat {
        id: 743_275,
        source_type: SourceType::Wikidata,
        name: "T.38",
        extensions: &["t38"],
        media_types: &["audio/t38", "image/t38"],
        signatures: &[],
        related_formats: &[],
    },
};

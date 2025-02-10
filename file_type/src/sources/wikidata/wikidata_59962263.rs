use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59962263: FileType = FileType {
    file_format: &FileFormat {
        id: 59_962_263,
        source_type: SourceType::Wikidata,
        name: "ChiWriter Document",
        extensions: &["chi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

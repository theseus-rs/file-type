use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967189: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_189,
        source_type: SourceType::Wikidata,
        name: "Fuzzac Packer module",
        extensions: &["fuz", "fuzz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

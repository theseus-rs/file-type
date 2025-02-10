use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_73675958: FileType = FileType {
    file_format: &FileFormat {
        id: 73_675_958,
        source_type: SourceType::Wikidata,
        name: "3M Printscape",
        extensions: &["psc", "std"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

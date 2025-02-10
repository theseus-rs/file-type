use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967211: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_211,
        source_type: SourceType::Wikidata,
        name: "Pumatracker module",
        extensions: &["puma"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

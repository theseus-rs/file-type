use crate::FileType;
use crate::format::{FileFormat, SourceType};

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

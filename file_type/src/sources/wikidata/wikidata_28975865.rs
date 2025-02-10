use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975865: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_865,
        source_type: SourceType::Wikidata,
        name: "OOGL VECT file",
        extensions: &["vect"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

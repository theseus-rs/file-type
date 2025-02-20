use crate::FileType;
use crate::format::{FileFormat, SourceType};

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

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_140447976: FileType = FileType {
    file_format: &FileFormat {
        id: 140_447_976,
        source_type: SourceType::Wikidata,
        name: "Ancestral Quest file format",
        extensions: &["aq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

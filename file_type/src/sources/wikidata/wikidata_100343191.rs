use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100343191: FileType = FileType {
    file_format: &FileFormat {
        id: 100_343_191,
        source_type: SourceType::Wikidata,
        name: "Corel Print House/Print Office Document, version 4",
        extensions: &["cpd", "cph", "cpo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

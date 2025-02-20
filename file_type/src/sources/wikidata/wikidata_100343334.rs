use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100343334: FileType = FileType {
    file_format: &FileFormat {
        id: 100_343_334,
        source_type: SourceType::Wikidata,
        name: "Corel Print House/Print Office Document, version 5",
        extensions: &["cpd", "cph", "cpo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

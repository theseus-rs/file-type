use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7271574: FileType = FileType {
    file_format: &FileFormat {
        id: 7_271_574,
        source_type: SourceType::Wikidata,
        name: "Quetzal file format",
        extensions: &["glksave", "sav"],
        media_types: &["application/x-glksave"],
        signatures: &[],
        related_formats: &[],
    },
};

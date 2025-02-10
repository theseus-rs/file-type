use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_99851769: FileType = FileType {
    file_format: &FileFormat {
        id: 99_851_769,
        source_type: SourceType::Wikidata,
        name: "ESRI Published Map Format",
        extensions: &["pmf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27349804: FileType = FileType {
    file_format: &FileFormat {
        id: 27_349_804,
        source_type: SourceType::Wikidata,
        name: "ESRI Arc/Info Binary Grid",
        extensions: &["adf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

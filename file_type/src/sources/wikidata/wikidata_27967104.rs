use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967104: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_104,
        source_type: SourceType::Wikidata,
        name: "Shroom Instrument",
        extensions: &["shi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

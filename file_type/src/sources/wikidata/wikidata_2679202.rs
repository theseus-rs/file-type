use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2679202: FileType = FileType {
    file_format: &FileFormat {
        id: 2_679_202,
        source_type: SourceType::Wikidata,
        name: "nds",
        extensions: &["nds"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

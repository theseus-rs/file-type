use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1124477: FileType = FileType {
    file_format: &FileFormat {
        id: 1_124_477,
        source_type: SourceType::Wikidata,
        name: "Efficient XML Interchange",
        extensions: &["exi"],
        media_types: &["application/exi"],
        signatures: &[],
        related_formats: &[],
    },
};

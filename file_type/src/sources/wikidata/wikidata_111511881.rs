use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111511881: FileType = FileType {
    file_format: &FileFormat {
        id: 111_511_881,
        source_type: SourceType::Wikidata,
        name: "ESRI ArcInfo Coverage Annotation File",
        extensions: &["txt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

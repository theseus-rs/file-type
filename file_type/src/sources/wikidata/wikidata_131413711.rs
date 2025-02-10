use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131413711: FileType = FileType {
    file_format: &FileFormat {
        id: 131_413_711,
        source_type: SourceType::Wikidata,
        name: "VisualProlog grammar file format",
        extensions: &["vipgrm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

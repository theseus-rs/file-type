use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60342537: FileType = FileType {
    file_format: &FileFormat {
        id: 60_342_537,
        source_type: SourceType::Wikidata,
        name: "SmartDraw format",
        extensions: &["sdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132527367: FileType = FileType {
    file_format: &FileFormat {
        id: 132_527_367,
        source_type: SourceType::Wikidata,
        name: "LEGO Mindstorms EV3 Data Logging Experiment file",
        extensions: &["ev3e"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

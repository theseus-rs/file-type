use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_55832374: FileType = FileType {
    file_format: &FileFormat {
        id: 55_832_374,
        source_type: SourceType::Wikidata,
        name: "Event Trace Log file format",
        extensions: &["etl"],
        media_types: &["application/x-ms-etl"],
        signatures: &[],
        related_formats: &[],
    },
};

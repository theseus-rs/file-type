use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100296675: FileType = FileType {
    file_format: &FileFormat {
        id: 100_296_675,
        source_type: SourceType::Wikidata,
        name: "Flow Charting file format, version I-II+",
        extensions: &["cht"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

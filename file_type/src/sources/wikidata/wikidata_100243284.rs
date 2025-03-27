use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100243284: FileType = FileType {
    file_format: &FileFormat {
        id: 100_243_284,
        source_type: SourceType::Wikidata,
        name: "Ichitaro Document",
        extensions: &["$td", "jtd", "jtt"],
        media_types: &["application/x-js-taro"],
        signatures: &[],
        related_formats: &[],
    },
};

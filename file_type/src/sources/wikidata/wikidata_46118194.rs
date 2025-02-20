use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_46118194: FileType = FileType {
    file_format: &FileFormat {
        id: 46_118_194,
        source_type: SourceType::Wikidata,
        name: "Lotus Approach View File, version 97",
        extensions: &["apr"],
        media_types: &["application/vnd.lotus-approach"],
        signatures: &[],
        related_formats: &[],
    },
};

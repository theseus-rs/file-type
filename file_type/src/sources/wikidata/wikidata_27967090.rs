use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967090: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_090,
        source_type: SourceType::Wikidata,
        name: "Epic Megagames MASI",
        extensions: &["psm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

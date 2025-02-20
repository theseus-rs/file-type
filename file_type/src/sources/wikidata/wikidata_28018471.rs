use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28018471: FileType = FileType {
    file_format: &FileFormat {
        id: 28_018_471,
        source_type: SourceType::Wikidata,
        name: "MPEG-2 program stream",
        extensions: &["mod", "mp2", "mpeg", "mpg"],
        media_types: &["video/MP2P"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122248584: FileType = FileType {
    file_format: &FileFormat {
        id: 122_248_584,
        source_type: SourceType::Wikidata,
        name: "YUV Video File",
        extensions: &["yuv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

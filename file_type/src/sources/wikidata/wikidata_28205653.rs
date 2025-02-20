use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205653: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_653,
        source_type: SourceType::Wikidata,
        name: "Abekas YUV",
        extensions: &["yuv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

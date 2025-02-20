use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125514786: FileType = FileType {
    file_format: &FileFormat {
        id: 125_514_786,
        source_type: SourceType::Wikidata,
        name: "Hasselblad RAW Image",
        extensions: &["fff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66244789: FileType = FileType {
    file_format: &FileFormat {
        id: 66_244_789,
        source_type: SourceType::Wikidata,
        name: "ScreenCam stand-alone Movies format",
        extensions: &["exe"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

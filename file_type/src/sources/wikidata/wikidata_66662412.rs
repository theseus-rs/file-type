use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66662412: FileType = FileType {
    file_format: &FileFormat {
        id: 66_662_412,
        source_type: SourceType::Wikidata,
        name: "ScreenCam Movies",
        extensions: &["scm"],
        media_types: &["application/vnd.lotus-screencam"],
        signatures: &[],
        related_formats: &[],
    },
};

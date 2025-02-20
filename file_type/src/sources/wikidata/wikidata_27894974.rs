use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27894974: FileType = FileType {
    file_format: &FileFormat {
        id: 27_894_974,
        source_type: SourceType::Wikidata,
        name: "Windows Media Redirector",
        extensions: &["wmx"],
        media_types: &["video/x-ms-wmx"],
        signatures: &[],
        related_formats: &[],
    },
};

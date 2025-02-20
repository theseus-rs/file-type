use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27895063: FileType = FileType {
    file_format: &FileFormat {
        id: 27_895_063,
        source_type: SourceType::Wikidata,
        name: "Windows Media Video",
        extensions: &["wm", "wmv"],
        media_types: &["video/x-ms-wmv"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105601274: FileType = FileType {
    file_format: &FileFormat {
        id: 105_601_274,
        source_type: SourceType::Wikidata,
        name: "X Window dump",
        extensions: &["xwd"],
        media_types: &["image/x-xwindowdump"],
        signatures: &[],
        related_formats: &[],
    },
};

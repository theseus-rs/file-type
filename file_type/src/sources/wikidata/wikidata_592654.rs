use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_592654: FileType = FileType {
    file_format: &FileFormat {
        id: 592_654,
        source_type: SourceType::Wikidata,
        name: "MPEG-1 Audio Layer II",
        extensions: &["mp2"],
        media_types: &["audio/MPA", "audio/mpeg"],
        signatures: &[],
        related_formats: &[],
    },
};

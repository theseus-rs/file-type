use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122169619: FileType = FileType {
    file_format: &FileFormat {
        id: 122_169_619,
        source_type: SourceType::Wikidata,
        name: "Task Container",
        extensions: &["rtc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

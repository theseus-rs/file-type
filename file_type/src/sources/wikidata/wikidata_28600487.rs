use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600487: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_487,
        source_type: SourceType::Wikidata,
        name: "Dalvik Executable",
        extensions: &["odex"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

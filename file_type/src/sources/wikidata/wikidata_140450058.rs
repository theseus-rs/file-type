use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_140450058: FileType = FileType {
    file_format: &FileFormat {
        id: 140_450_058,
        source_type: SourceType::Wikidata,
        name: "My Family Tree DSR file format",
        extensions: &["tre"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128694058: FileType = FileType {
    file_format: &FileFormat {
        id: 128_694_058,
        source_type: SourceType::Wikidata,
        name: "OpenBugs file",
        extensions: &["bug"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

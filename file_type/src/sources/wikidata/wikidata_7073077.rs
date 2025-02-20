use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7073077: FileType = FileType {
    file_format: &FileFormat {
        id: 7_073_077,
        source_type: SourceType::Wikidata,
        name: "OTA bitmap",
        extensions: &["otb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133042215: FileType = FileType {
    file_format: &FileFormat {
        id: 133_042_215,
        source_type: SourceType::Wikidata,
        name: "askSam Document for Windows 2-3",
        extensions: &["ask"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

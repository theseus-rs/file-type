use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133042568: FileType = FileType {
    file_format: &FileFormat {
        id: 133_042_568,
        source_type: SourceType::Wikidata,
        name: "askSam Document for Windows 4-7",
        extensions: &["ask"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117834959: FileType = FileType {
    file_format: &FileFormat {
        id: 117_834_959,
        source_type: SourceType::Wikidata,
        name: "Brooktrout Fax-Mail 96 file",
        extensions: &["brk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

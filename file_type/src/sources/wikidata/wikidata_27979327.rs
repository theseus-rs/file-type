use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27979327: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_327,
        source_type: SourceType::Wikidata,
        name: "Advanced Function Presentation",
        extensions: &["afp"],
        media_types: &["application/x-afp"],
        signatures: &[],
        related_formats: &[],
    },
};

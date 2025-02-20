use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207227: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_227,
        source_type: SourceType::Wikidata,
        name: "RIFF DIB",
        extensions: &["rdi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

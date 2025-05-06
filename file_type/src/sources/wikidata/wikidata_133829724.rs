use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133829724: FileType = FileType {
    file_format: &FileFormat {
        id: 133_829_724,
        source_type: SourceType::Wikidata,
        name: "PETSCII screen code sequence",
        extensions: &["seq"],
        media_types: &["text/x-petscii-sequence"],
        signatures: &[],
        related_formats: &[],
    },
};

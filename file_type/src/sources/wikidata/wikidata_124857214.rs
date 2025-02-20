use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_124857214: FileType = FileType {
    file_format: &FileFormat {
        id: 124_857_214,
        source_type: SourceType::Wikidata,
        name: "OCR results file",
        extensions: &["orf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

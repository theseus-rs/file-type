use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27959858: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_858,
        source_type: SourceType::Wikidata,
        name: "Make-A-Melody song file",
        extensions: &["mus"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

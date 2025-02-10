use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_86995619: FileType = FileType {
    file_format: &FileFormat {
        id: 86_995_619,
        source_type: SourceType::Wikidata,
        name: "PaperPort MAX 8-12",
        extensions: &["max"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135227906: FileType = FileType {
    file_format: &FileFormat {
        id: 135_227_906,
        source_type: SourceType::Wikidata,
        name: "gitignore",
        extensions: &["gitignore"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

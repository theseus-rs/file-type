use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111166086: FileType = FileType {
    file_format: &FileFormat {
        id: 111_166_086,
        source_type: SourceType::Wikidata,
        name: "Songbase File",
        extensions: &["sngbase"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

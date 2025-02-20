use crate::FileType;
use crate::format::{FileFormat, SourceType};

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

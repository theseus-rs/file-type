use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66759482: FileType = FileType {
    file_format: &FileFormat {
        id: 66_759_482,
        source_type: SourceType::Wikidata,
        name: "InfoPath Form file",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

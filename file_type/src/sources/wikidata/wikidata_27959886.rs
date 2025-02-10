use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27959886: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_886,
        source_type: SourceType::Wikidata,
        name: "Cubase song",
        extensions: &["all"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

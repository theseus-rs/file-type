use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_62571475: FileType = FileType {
    file_format: &FileFormat {
        id: 62_571_475,
        source_type: SourceType::Wikidata,
        name: "Online Description Tool Format",
        extensions: &["odt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

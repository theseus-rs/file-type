use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_62414875: FileType = FileType {
    file_format: &FileFormat {
        id: 62_414_875,
        source_type: SourceType::Wikidata,
        name: "XAML Binary Format",
        extensions: &["xbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

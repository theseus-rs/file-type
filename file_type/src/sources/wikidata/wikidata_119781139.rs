use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119781139: FileType = FileType {
    file_format: &FileFormat {
        id: 119_781_139,
        source_type: SourceType::Wikidata,
        name: "Street Atlas USA File",
        extensions: &["saf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

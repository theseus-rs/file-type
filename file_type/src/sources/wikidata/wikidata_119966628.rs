use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119966628: FileType = FileType {
    file_format: &FileFormat {
        id: 119_966_628,
        source_type: SourceType::Wikidata,
        name: "Pocket Streets Map",
        extensions: &["mps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111356337: FileType = FileType {
    file_format: &FileFormat {
        id: 111_356_337,
        source_type: SourceType::Wikidata,
        name: "Turtle Beach WaveFront drum set format",
        extensions: &["wfd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

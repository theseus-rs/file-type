use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111356350: FileType = FileType {
    file_format: &FileFormat {
        id: 111_356_350,
        source_type: SourceType::Wikidata,
        name: "Turtle Beach WaveFront program format",
        extensions: &["wfp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

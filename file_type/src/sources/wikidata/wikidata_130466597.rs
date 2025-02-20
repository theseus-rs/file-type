use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130466597: FileType = FileType {
    file_format: &FileFormat {
        id: 130_466_597,
        source_type: SourceType::Wikidata,
        name: "Parsing Expression Grammar file format",
        extensions: &["peg"],
        media_types: &["text/x-peg"],
        signatures: &[],
        related_formats: &[],
    },
};

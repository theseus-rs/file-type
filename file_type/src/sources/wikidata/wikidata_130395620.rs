use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130395620: FileType = FileType {
    file_format: &FileFormat {
        id: 130_395_620,
        source_type: SourceType::Wikidata,
        name: "Octave source code file",
        extensions: &["m"],
        media_types: &["text/octave"],
        signatures: &[],
        related_formats: &[],
    },
};

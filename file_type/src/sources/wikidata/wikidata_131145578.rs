use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131145578: FileType = FileType {
    file_format: &FileFormat {
        id: 131_145_578,
        source_type: SourceType::Wikidata,
        name: "Spice source file format",
        extensions: &["spice"],
        media_types: &["text/x-spice"],
        signatures: &[],
        related_formats: &[],
    },
};

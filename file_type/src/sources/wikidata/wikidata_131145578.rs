use crate::FileType;
use crate::format::{FileFormat, SourceType};

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

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_17149623: FileType = FileType {
    file_format: &FileFormat {
        id: 17_149_623,
        source_type: SourceType::Wikidata,
        name: "OpenFlight",
        extensions: &[],
        media_types: &["model/flt"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_112652534: FileType = FileType {
    file_format: &FileFormat {
        id: 112_652_534,
        source_type: SourceType::Wikidata,
        name: "Astound 1.5 Library file format",
        extensions: &["asl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

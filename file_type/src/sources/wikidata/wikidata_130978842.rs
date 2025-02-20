use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130978842: FileType = FileType {
    file_format: &FileFormat {
        id: 130_978_842,
        source_type: SourceType::Wikidata,
        name: "Slash file format",
        extensions: &["sla"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

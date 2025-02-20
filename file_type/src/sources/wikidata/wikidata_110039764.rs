use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110039764: FileType = FileType {
    file_format: &FileFormat {
        id: 110_039_764,
        source_type: SourceType::Wikidata,
        name: "Timeline Maker Document",
        extensions: &["tlm", "tlm3", "tlm4", "tlmp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

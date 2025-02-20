use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110039945: FileType = FileType {
    file_format: &FileFormat {
        id: 110_039_945,
        source_type: SourceType::Wikidata,
        name: "Phantom CINE Video File",
        extensions: &["cin", "cine"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

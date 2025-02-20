use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27959878: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_878,
        source_type: SourceType::Wikidata,
        name: "Piston Collage song",
        extensions: &["ptcop", "pttune"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

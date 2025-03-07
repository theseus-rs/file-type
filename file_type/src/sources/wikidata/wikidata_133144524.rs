use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133144524: FileType = FileType {
    file_format: &FileFormat {
        id: 133_144_524,
        source_type: SourceType::Wikidata,
        name: "Codebook Exchange Format",
        extensions: &["qdc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

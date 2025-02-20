use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111191629: FileType = FileType {
    file_format: &FileFormat {
        id: 111_191_629,
        source_type: SourceType::Wikidata,
        name: "Viewpoint format",
        extensions: &["vet"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

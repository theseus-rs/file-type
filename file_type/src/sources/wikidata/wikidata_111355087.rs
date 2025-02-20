use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111355087: FileType = FileType {
    file_format: &FileFormat {
        id: 111_355_087,
        source_type: SourceType::Wikidata,
        name: "G.711 mu-law US telephony format",
        extensions: &["ulw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_737207: FileType = FileType {
    file_format: &FileFormat {
        id: 737_207,
        source_type: SourceType::Wikidata,
        name: "RealVideo",
        extensions: &["rv"],
        media_types: &["application/vnd.rn-realmedia"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_22097440: FileType = FileType {
    file_format: &FileFormat {
        id: 22_097_440,
        source_type: SourceType::Wikidata,
        name: "IPSW",
        extensions: &["ipsw"],
        media_types: &["application/x-itunes-ipsw"],
        signatures: &[],
        related_formats: &[],
    },
};

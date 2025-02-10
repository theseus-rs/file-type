use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34745227: FileType = FileType {
    file_format: &FileFormat {
        id: 34_745_227,
        source_type: SourceType::Wikidata,
        name: "Spline Font Database",
        extensions: &["sfd"],
        media_types: &["application/vnd.font-fontforge-sfd"],
        signatures: &[],
        related_formats: &[],
    },
};

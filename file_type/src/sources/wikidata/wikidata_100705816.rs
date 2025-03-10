use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100705816: FileType = FileType {
    file_format: &FileFormat {
        id: 100_705_816,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Document 1-2",
        extensions: &[],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100706066: FileType = FileType {
    file_format: &FileFormat {
        id: 100_706_066,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Document 3.2",
        extensions: &[],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};

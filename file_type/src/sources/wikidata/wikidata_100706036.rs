use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100706036: FileType = FileType {
    file_format: &FileFormat {
        id: 100_706_036,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Document 3",
        extensions: &[],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};

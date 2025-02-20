use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_89682010: FileType = FileType {
    file_format: &FileFormat {
        id: 89_682_010,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Document 5",
        extensions: &["qwd", "qxd", "qxt"],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};

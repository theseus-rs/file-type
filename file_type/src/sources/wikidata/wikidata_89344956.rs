use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_89344956: FileType = FileType {
    file_format: &FileFormat {
        id: 89_344_956,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Document 3.3",
        extensions: &["qwd", "qxd", "qxt"],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};

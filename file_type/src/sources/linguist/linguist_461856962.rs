use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_461856962: FileType = FileType {
    file_format: &FileFormat {
        id: 461_856_962,
        source_type: SourceType::Linguist,
        name: "Nunjucks",
        extensions: &["njk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

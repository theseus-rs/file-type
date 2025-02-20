use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_930281: FileType = FileType {
    file_format: &FileFormat {
        id: 930_281,
        source_type: SourceType::Wikidata,
        name: "Windows thumbnail cache",
        extensions: &["db"],
        media_types: &["application/vnd.microsoft.windows.thumbnail-cache"],
        signatures: &[],
        related_formats: &[],
    },
};

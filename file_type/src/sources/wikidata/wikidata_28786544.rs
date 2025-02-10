use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28786544: FileType = FileType {
    file_format: &FileFormat {
        id: 28_786_544,
        source_type: SourceType::Wikidata,
        name: "Netscape bookmarks",
        extensions: &["htm", "html"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

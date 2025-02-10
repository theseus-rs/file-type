use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979401: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_401,
        source_type: SourceType::Wikidata,
        name: "JP2",
        extensions: &["jp2"],
        media_types: &["image/jp2"],
        signatures: &[],
        related_formats: &[],
    },
};

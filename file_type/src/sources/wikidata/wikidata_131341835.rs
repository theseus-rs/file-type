use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131341835: FileType = FileType {
    file_format: &FileFormat {
        id: 131_341_835,
        source_type: SourceType::Wikidata,
        name: "UrbiScript source code file",
        extensions: &["u"],
        media_types: &["application/x-urbiscript"],
        signatures: &[],
        related_formats: &[],
    },
};

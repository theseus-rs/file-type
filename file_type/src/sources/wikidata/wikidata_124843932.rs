use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_124843932: FileType = FileType {
    file_format: &FileFormat {
        id: 124_843_932,
        source_type: SourceType::Wikidata,
        name: "Apple Contacts Archive file",
        extensions: &["abbu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

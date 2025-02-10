use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_85513175: FileType = FileType {
    file_format: &FileFormat {
        id: 85_513_175,
        source_type: SourceType::Wikidata,
        name: "Cindex Document, version 2",
        extensions: &["cdx", "tpl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7271522: FileType = FileType {
    file_format: &FileFormat {
        id: 7_271_522,
        source_type: SourceType::Wikidata,
        name: "Question Object File Format",
        extensions: &["quiz", "quox"],
        media_types: &["application/vnd.quobject-quoxdocument"],
        signatures: &[],
        related_formats: &[],
    },
};

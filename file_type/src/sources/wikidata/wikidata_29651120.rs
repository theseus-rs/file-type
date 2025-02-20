use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29651120: FileType = FileType {
    file_format: &FileFormat {
        id: 29_651_120,
        source_type: SourceType::Wikidata,
        name: "Personal Folder File",
        extensions: &["pst"],
        media_types: &["application/vnd.ms-outlook"],
        signatures: &[],
        related_formats: &[],
    },
};

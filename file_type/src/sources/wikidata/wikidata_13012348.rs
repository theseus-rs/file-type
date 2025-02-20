use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_13012348: FileType = FileType {
    file_format: &FileFormat {
        id: 13_012_348,
        source_type: SourceType::Wikidata,
        name: "Adobe Flash project",
        extensions: &["fla"],
        media_types: &["application/vnd.adobe.fla"],
        signatures: &[],
        related_formats: &[],
    },
};

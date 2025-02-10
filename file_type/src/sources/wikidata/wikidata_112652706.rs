use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112652706: FileType = FileType {
    file_format: &FileFormat {
        id: 112_652_706,
        source_type: SourceType::Wikidata,
        name: "Astound Video Project file",
        extensions: &["prj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

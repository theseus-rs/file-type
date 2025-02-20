use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_109944989: FileType = FileType {
    file_format: &FileFormat {
        id: 109_944_989,
        source_type: SourceType::Wikidata,
        name: "Ulead Template File",
        extensions: &["tpl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111262682: FileType = FileType {
    file_format: &FileFormat {
        id: 111_262_682,
        source_type: SourceType::Wikidata,
        name: "Yamaha A3000 sample file",
        extensions: &["a3s"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

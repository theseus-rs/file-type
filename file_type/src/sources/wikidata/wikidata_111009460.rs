use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111009460: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_460,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Calendar File format",
        extensions: &["cal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

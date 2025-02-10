use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66811836: FileType = FileType {
    file_format: &FileFormat {
        id: 66_811_836,
        source_type: SourceType::Wikidata,
        name: "Inform source code file",
        extensions: &["inf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

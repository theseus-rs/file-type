use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27960128: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_128,
        source_type: SourceType::Wikidata,
        name: "Computerized Speech Lab NSP",
        extensions: &["nsp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

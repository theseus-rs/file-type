use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3045823: FileType = FileType {
    file_format: &FileFormat {
        id: 3_045_823,
        source_type: SourceType::Wikidata,
        name: "eBox",
        extensions: &["ebx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

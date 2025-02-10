use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967181: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_181,
        source_type: SourceType::Wikidata,
        name: "Farandole Composer pattern",
        extensions: &["fpt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

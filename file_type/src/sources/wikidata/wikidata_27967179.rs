use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967179: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_179,
        source_type: SourceType::Wikidata,
        name: "Farandole Form 2.0",
        extensions: &["f2r"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

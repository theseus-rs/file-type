use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114877040: FileType = FileType {
    file_format: &FileFormat {
        id: 114_877_040,
        source_type: SourceType::Wikidata,
        name: "Microsoft Money Backup File",
        extensions: &["mbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

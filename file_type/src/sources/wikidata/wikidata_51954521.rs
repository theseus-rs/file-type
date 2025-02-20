use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51954521: FileType = FileType {
    file_format: &FileFormat {
        id: 51_954_521,
        source_type: SourceType::Wikidata,
        name: "Microsoft FoxPro Database, version 2.6",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

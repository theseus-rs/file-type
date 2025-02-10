use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48021763: FileType = FileType {
    file_format: &FileFormat {
        id: 48_021_763,
        source_type: SourceType::Wikidata,
        name: "Microsoft Access Database, version 2002",
        extensions: &["mdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

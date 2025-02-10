use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48021588: FileType = FileType {
    file_format: &FileFormat {
        id: 48_021_588,
        source_type: SourceType::Wikidata,
        name: "Microsoft Access Database, version 2000",
        extensions: &["mdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61984337: FileType = FileType {
    file_format: &FileFormat {
        id: 61_984_337,
        source_type: SourceType::Wikidata,
        name: "Microsoft Visual FoxPro database container (table files)",
        extensions: &["dbc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

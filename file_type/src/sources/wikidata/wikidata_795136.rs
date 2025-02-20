use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_795136: FileType = FileType {
    file_format: &FileFormat {
        id: 795_136,
        source_type: SourceType::Wikidata,
        name: "Alembic",
        extensions: &["abc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

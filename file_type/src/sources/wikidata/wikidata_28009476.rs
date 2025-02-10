use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28009476: FileType = FileType {
    file_format: &FileFormat {
        id: 28_009_476,
        source_type: SourceType::Wikidata,
        name: "SCF",
        extensions: &["scf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

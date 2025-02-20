use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27966894: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_894,
        source_type: SourceType::Wikidata,
        name: "GSF",
        extensions: &["gsf", "gsflib", "minigsf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

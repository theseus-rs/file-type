use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111665313: FileType = FileType {
    file_format: &FileFormat {
        id: 111_665_313,
        source_type: SourceType::Wikidata,
        name: "AbiWord Gzip Compressed Document",
        extensions: &["zabw"],
        media_types: &["application/abiword"],
        signatures: &[],
        related_formats: &[],
    },
};

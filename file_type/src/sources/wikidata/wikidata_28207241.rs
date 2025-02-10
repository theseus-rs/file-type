use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207241: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_241,
        source_type: SourceType::Wikidata,
        name: "SBIG CCDOPS image",
        extensions: &["sbig"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

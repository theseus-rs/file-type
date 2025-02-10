use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979404: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_404,
        source_type: SourceType::Wikidata,
        name: "PICS",
        extensions: &["pcs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

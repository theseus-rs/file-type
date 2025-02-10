use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_17092932: FileType = FileType {
    file_format: &FileFormat {
        id: 17_092_932,
        source_type: SourceType::Wikidata,
        name: "JPEG-XT",
        extensions: &["jpeg", "jpg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

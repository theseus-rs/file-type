use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122021046: FileType = FileType {
    file_format: &FileFormat {
        id: 122_021_046,
        source_type: SourceType::Wikidata,
        name: "Retina file",
        extensions: &["rtd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27979400: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_400,
        source_type: SourceType::Wikidata,
        name: "JPX",
        extensions: &["jpf", "jpx"],
        media_types: &["image/jpx"],
        signatures: &[],
        related_formats: &[],
    },
};

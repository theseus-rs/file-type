use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_46979481: FileType = FileType {
    file_format: &FileFormat {
        id: 46_979_481,
        source_type: SourceType::Wikidata,
        name: "Universal Windows Platform application package",
        extensions: &["appx"],
        media_types: &["application/appx"],
        signatures: &[],
        related_formats: &[],
    },
};

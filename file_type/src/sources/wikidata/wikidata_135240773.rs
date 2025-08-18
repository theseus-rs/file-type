use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135240773: FileType = FileType {
    file_format: &FileFormat {
        id: 135_240_773,
        source_type: SourceType::Wikidata,
        name: "PMX file",
        extensions: &["pmx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

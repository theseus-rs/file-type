use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111418374: FileType = FileType {
    file_format: &FileFormat {
        id: 111_418_374,
        source_type: SourceType::Wikidata,
        name: "Adobe Bridge Cache File",
        extensions: &["bc", "bcm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

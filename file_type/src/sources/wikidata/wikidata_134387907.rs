use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134387907: FileType = FileType {
    file_format: &FileFormat {
        id: 134_387_907,
        source_type: SourceType::Wikidata,
        name: "SOFA Scene file",
        extensions: &["scn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

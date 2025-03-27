use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66741424: FileType = FileType {
    file_format: &FileFormat {
        id: 66_741_424,
        source_type: SourceType::Wikidata,
        name: "Universal Windows Platform application package bundle",
        extensions: &["appxbundle"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

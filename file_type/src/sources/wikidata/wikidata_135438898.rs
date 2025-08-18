use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135438898: FileType = FileType {
    file_format: &FileFormat {
        id: 135_438_898,
        source_type: SourceType::Wikidata,
        name: "Zope page template file",
        extensions: &["zpt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

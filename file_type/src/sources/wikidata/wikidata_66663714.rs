use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66663714: FileType = FileType {
    file_format: &FileFormat {
        id: 66_663_714,
        source_type: SourceType::Wikidata,
        name: "Hewlett Packard Graphics Gallery",
        extensions: &["gal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

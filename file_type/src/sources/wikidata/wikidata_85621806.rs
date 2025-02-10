use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_85621806: FileType = FileType {
    file_format: &FileFormat {
        id: 85_621_806,
        source_type: SourceType::Wikidata,
        name: "PFS:First Choice Document 3",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

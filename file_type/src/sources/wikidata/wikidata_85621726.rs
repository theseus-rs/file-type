use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_85621726: FileType = FileType {
    file_format: &FileFormat {
        id: 85_621_726,
        source_type: SourceType::Wikidata,
        name: "PFS:First Choice Document",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

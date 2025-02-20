use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111333907: FileType = FileType {
    file_format: &FileFormat {
        id: 111_333_907,
        source_type: SourceType::Wikidata,
        name: "Roland MT-32 Control + PCM ROM dumps",
        extensions: &["rom"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

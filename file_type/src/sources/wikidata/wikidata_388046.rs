use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_388046: FileType = FileType {
    file_format: &FileFormat {
        id: 388_046,
        source_type: SourceType::Wikidata,
        name: "X-Face",
        extensions: &["face", "xface"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

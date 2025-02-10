use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27826386: FileType = FileType {
    file_format: &FileFormat {
        id: 27_826_386,
        source_type: SourceType::Wikidata,
        name: "OVR pyramid file",
        extensions: &["ovr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

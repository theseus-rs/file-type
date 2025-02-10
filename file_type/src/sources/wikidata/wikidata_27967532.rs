use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967532: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_532,
        source_type: SourceType::Wikidata,
        name: "DVD Information File",
        extensions: &["bup", "ifo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

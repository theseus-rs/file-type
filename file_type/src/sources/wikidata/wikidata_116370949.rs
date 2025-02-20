use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116370949: FileType = FileType {
    file_format: &FileFormat {
        id: 116_370_949,
        source_type: SourceType::Wikidata,
        name: "DFPWM",
        extensions: &["dfpwm"],
        media_types: &["audio/dfpwm"],
        signatures: &[],
        related_formats: &[],
    },
};

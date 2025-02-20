use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207355: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_355,
        source_type: SourceType::Wikidata,
        name: "TrueVista",
        extensions: &["vst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

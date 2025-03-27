use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205384: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_384,
        source_type: SourceType::Wikidata,
        name: "Mamiya MEF",
        extensions: &["mef"],
        media_types: &["image/x-raw-mamiya"],
        signatures: &[],
        related_formats: &[],
    },
};

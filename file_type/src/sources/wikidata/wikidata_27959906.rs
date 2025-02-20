use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27959906: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_906,
        source_type: SourceType::Wikidata,
        name: "Super Studio Session song",
        extensions: &["sss"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126086338: FileType = FileType {
    file_format: &FileFormat {
        id: 126_086_338,
        source_type: SourceType::Wikidata,
        name: "IMF Package Packing List",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

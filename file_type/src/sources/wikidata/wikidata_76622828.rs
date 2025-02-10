use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_76622828: FileType = FileType {
    file_format: &FileFormat {
        id: 76_622_828,
        source_type: SourceType::Wikidata,
        name: "WOLF eBook",
        extensions: &["wol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

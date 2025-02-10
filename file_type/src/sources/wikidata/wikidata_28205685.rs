use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205685: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_685,
        source_type: SourceType::Wikidata,
        name: "AMOS Picture Bank",
        extensions: &["abk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

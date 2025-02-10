use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47916351: FileType = FileType {
    file_format: &FileFormat {
        id: 47_916_351,
        source_type: SourceType::Wikidata,
        name: "Ventura Publisher Vector Graphics",
        extensions: &["gem"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

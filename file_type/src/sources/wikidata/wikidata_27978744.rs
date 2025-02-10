use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27978744: FileType = FileType {
    file_format: &FileFormat {
        id: 27_978_744,
        source_type: SourceType::Wikidata,
        name: "DeluxePaint Animation",
        extensions: &["anm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71859659: FileType = FileType {
    file_format: &FileFormat {
        id: 71_859_659,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing, version X4",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

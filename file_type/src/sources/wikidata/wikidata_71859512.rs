use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71859512: FileType = FileType {
    file_format: &FileFormat {
        id: 71_859_512,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing, version X3",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

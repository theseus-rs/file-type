use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71858982: FileType = FileType {
    file_format: &FileFormat {
        id: 71_858_982,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing, version 10",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

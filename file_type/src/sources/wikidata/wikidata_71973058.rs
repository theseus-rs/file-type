use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71973058: FileType = FileType {
    file_format: &FileFormat {
        id: 71_973_058,
        source_type: SourceType::Wikidata,
        name: "CorelDraw Drawing, version X5",
        extensions: &["cdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

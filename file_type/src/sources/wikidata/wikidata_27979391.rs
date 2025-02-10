use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979391: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_391,
        source_type: SourceType::Wikidata,
        name: "ANSI Animation",
        extensions: &["ans", "vt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

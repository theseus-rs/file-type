use crate::FileType;
use crate::format::{FileFormat, SourceType};

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

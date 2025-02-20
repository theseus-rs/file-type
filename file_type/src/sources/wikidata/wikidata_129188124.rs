use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129188124: FileType = FileType {
    file_format: &FileFormat {
        id: 129_188_124,
        source_type: SourceType::Wikidata,
        name: "FreeFem++ source code file",
        extensions: &["edp"],
        media_types: &["text/x-freefem"],
        signatures: &[],
        related_formats: &[],
    },
};

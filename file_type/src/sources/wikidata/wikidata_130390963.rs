use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130390963: FileType = FileType {
    file_format: &FileFormat {
        id: 130_390_963,
        source_type: SourceType::Wikidata,
        name: "Objective-C++ source code file",
        extensions: &["mm"],
        media_types: &["text/x-objective-c++"],
        signatures: &[],
        related_formats: &[],
    },
};

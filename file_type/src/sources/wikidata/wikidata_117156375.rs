use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117156375: FileType = FileType {
    file_format: &FileFormat {
        id: 117_156_375,
        source_type: SourceType::Wikidata,
        name: "Pyro data disc project file",
        extensions: &["cwd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

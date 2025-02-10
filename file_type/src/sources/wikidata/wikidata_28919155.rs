use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28919155: FileType = FileType {
    file_format: &FileFormat {
        id: 28_919_155,
        source_type: SourceType::Wikidata,
        name: "Rhino Worksession",
        extensions: &["rws"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125692441: FileType = FileType {
    file_format: &FileFormat {
        id: 125_692_441,
        source_type: SourceType::Wikidata,
        name: "Microsoft PowerPoint Presentation Template",
        extensions: &["potx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

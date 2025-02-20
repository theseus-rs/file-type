use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125692387: FileType = FileType {
    file_format: &FileFormat {
        id: 125_692_387,
        source_type: SourceType::Wikidata,
        name: "Microsoft PowerPoint 2007 XML Template",
        extensions: &["potm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

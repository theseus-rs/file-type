use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125823522: FileType = FileType {
    file_format: &FileFormat {
        id: 125_823_522,
        source_type: SourceType::Wikidata,
        name: "Microsoft Help Attribute Definition file",
        extensions: &["hxw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

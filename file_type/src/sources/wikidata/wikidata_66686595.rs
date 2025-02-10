use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66686595: FileType = FileType {
    file_format: &FileFormat {
        id: 66_686_595,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Template file format",
        extensions: &["wpt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

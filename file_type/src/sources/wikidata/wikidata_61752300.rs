use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_61752300: FileType = FileType {
    file_format: &FileFormat {
        id: 61_752_300,
        source_type: SourceType::Wikidata,
        name: "Microsoft FrontPage file format",
        extensions: &["lck"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

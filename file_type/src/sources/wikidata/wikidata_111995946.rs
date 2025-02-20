use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111995946: FileType = FileType {
    file_format: &FileFormat {
        id: 111_995_946,
        source_type: SourceType::Wikidata,
        name: "Microsoft Paint File",
        extensions: &["msp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

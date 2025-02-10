use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51841970: FileType = FileType {
    file_format: &FileFormat {
        id: 51_841_970,
        source_type: SourceType::Wikidata,
        name: "Microsoft FoxPro Library",
        extensions: &["plb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

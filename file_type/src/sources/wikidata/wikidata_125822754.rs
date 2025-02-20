use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125822754: FileType = FileType {
    file_format: &FileFormat {
        id: 125_822_754,
        source_type: SourceType::Wikidata,
        name: "Microsoft Help Index",
        extensions: &["chi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

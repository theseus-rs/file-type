use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_60372734: FileType = FileType {
    file_format: &FileFormat {
        id: 60_372_734,
        source_type: SourceType::Wikidata,
        name: "Quark Xpress Data File, version 6",
        extensions: &["qcd", "qct", "qtp", "qxp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

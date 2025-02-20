use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60371646: FileType = FileType {
    file_format: &FileFormat {
        id: 60_371_646,
        source_type: SourceType::Wikidata,
        name: "Quark Xpress Data File, version 10",
        extensions: &["qcd", "qct", "qtp", "qxp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

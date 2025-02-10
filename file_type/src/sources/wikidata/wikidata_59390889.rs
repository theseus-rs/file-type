use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59390889: FileType = FileType {
    file_format: &FileFormat {
        id: 59_390_889,
        source_type: SourceType::Wikidata,
        name: "GraphPad Prism file format, version 4",
        extensions: &["pzf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

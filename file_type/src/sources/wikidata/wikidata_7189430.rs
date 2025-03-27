use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7189430: FileType = FileType {
    file_format: &FileFormat {
        id: 7_189_430,
        source_type: SourceType::Wikidata,
        name: "PhyloXML",
        extensions: &["phyloxml"],
        media_types: &["text/x-phyloxml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};

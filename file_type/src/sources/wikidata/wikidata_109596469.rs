use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109596469: FileFormat = FileFormat {
    id: 109_596_469,
    source_type: SourceType::Wikidata,
    name: "DrawPlus Template",
    extensions: &["dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863113: FileFormat = FileFormat {
    id: 105_863_113,
    source_type: SourceType::Wikidata,
    name: "mzXML",
    extensions: &["mzXML"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};

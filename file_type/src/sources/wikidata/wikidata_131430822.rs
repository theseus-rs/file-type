use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131430822: FileFormat = FileFormat {
    id: 131_430_822,
    source_type: SourceType::Wikidata,
    name: "XQuery Source File",
    extensions: &["xq", "xql", "xqm", "xquery", "xqy"],
    media_types: &["application/xquery", "text/xquery"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50183465: FileFormat = FileFormat {
    id: 50_183_465,
    source_type: SourceType::Wikidata,
    name: "AXD HTTP Handler File",
    extensions: &["axd"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};

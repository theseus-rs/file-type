use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50183465: FileFormat = FileFormat {
    id: 50_183_465,
    source_type: SourceType::Wikidata,
    name: "AXD HTTP Handler File",
    extensions: &["axd"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};

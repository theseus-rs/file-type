use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124622467: FileFormat = FileFormat {
    id: 124_622_467,
    source_type: SourceType::Wikidata,
    name: "TEI/XML",
    extensions: &["odd", "xml"],
    media_types: &["application/tei+xml"],
    internal_signatures: &[],
    related_formats: &[],
};

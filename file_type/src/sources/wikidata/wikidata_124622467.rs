use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124622467: FileFormat = FileFormat {
    id: 124_622_467,
    source_type: SourceType::Wikidata,
    name: "TEI/XML",
    extensions: &["odd", "xml"],
    media_types: &["application/tei+xml"],
    signatures: &[],
    related_formats: &[],
};

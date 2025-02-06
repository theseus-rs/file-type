use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2044200: FileFormat = FileFormat {
    id: 2_044_200,
    source_type: SourceType::Wikidata,
    name: "PICT",
    extensions: &["pct", "pict"],
    media_types: &["image/x-pict"],
    signatures: &[],
    related_formats: &[],
};

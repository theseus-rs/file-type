use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117813268: FileFormat = FileFormat {
    id: 117_813_268,
    source_type: SourceType::Wikidata,
    name: "Meter Data",
    extensions: &["dta"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600763: FileFormat = FileFormat {
    id: 28_600_763,
    source_type: SourceType::Wikidata,
    name: "Electronic Arts INF",
    extensions: &["inf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

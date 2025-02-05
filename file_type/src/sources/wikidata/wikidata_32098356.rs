use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_32098356: FileFormat = FileFormat {
    id: 32_098_356,
    source_type: SourceType::Wikidata,
    name: "GENMIDI.OP2 OPL2 sound bank",
    extensions: &["op2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

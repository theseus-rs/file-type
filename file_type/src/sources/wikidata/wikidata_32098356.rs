use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_32098356: FileFormat = FileFormat {
    id: 32_098_356,
    source_type: SourceType::Wikidata,
    name: "GENMIDI.OP2 OPL2 sound bank",
    extensions: &["op2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

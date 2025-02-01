use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_32098356: FileFormat = FileFormat {
    id: 32_098_356,
    puid: "wikidata/32098356",
    name: "GENMIDI.OP2 OPL2 sound bank",
    extensions: &["op2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

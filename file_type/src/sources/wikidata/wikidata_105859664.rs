use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859664: FileFormat = FileFormat {
    id: 105_859_664,
    puid: "wikidata/105859664",
    name: "Verilog source code (with pre)",
    extensions: &["v"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};

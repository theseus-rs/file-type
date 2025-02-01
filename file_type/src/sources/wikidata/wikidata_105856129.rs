use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856129: FileFormat = FileFormat {
    id: 105_856_129,
    puid: "wikidata/105856129",
    name: "Delphi Project source (with rem)",
    extensions: &["dpr"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};

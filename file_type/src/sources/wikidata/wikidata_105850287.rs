use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850287: FileFormat = FileFormat {
    id: 105_850_287,
    puid: "wikidata/105850287",
    name: "CryEngine Project (v5)",
    extensions: &["cryproject"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};

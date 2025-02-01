use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850591: FileFormat = FileFormat {
    id: 105_850_591,
    puid: "wikidata/105850591",
    name: "CryEngine Project (generic)",
    extensions: &["cryproject"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};

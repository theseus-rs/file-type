use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967108: FileFormat = FileFormat {
    id: 27_967_108,
    puid: "wikidata/27967108",
    name: "STOS memory bank",
    extensions: &["mbk", "mbs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58367808: FileFormat = FileFormat {
    id: 58_367_808,
    puid: "wikidata/58367808",
    name: "BSDIFF",
    extensions: &["bsdiff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

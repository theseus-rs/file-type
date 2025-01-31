use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47519802: FileFormat = FileFormat {
    id: 47_519_802,
    puid: "wikidata/47519802",
    name: "Serif PagePlus Publication file format (generic)",
    extensions: &["ppp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122169761: FileFormat = FileFormat {
    id: 122_169_761,
    puid: "wikidata/122169761",
    name: "Domain Cached Credentials",
    extensions: &["dcc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

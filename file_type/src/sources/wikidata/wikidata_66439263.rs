use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66439263: FileFormat = FileFormat {
    id: 66_439_263,
    puid: "wikidata/66439263",
    name: "Word Perfect Templates file format",
    extensions: &["wpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

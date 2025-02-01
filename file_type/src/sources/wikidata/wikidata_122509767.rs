use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122509767: FileFormat = FileFormat {
    id: 122_509_767,
    puid: "wikidata/122509767",
    name: "Pretty Good Privacy (PGP) Groups Data",
    extensions: &["pgr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

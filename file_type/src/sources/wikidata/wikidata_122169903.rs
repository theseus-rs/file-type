use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122169903: FileFormat = FileFormat {
    id: 122_169_903,
    puid: "wikidata/122169903",
    name: "PPA and PWDUMP Password Hashes",
    extensions: &["hdc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

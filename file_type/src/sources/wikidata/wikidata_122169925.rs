use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122169925: FileFormat = FileFormat {
    id: 122_169_925,
    puid: "wikidata/122169925",
    name: "GPU PWDUMP Password Hashes (NTLM)",
    extensions: &["pwdump"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

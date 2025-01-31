use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857104: FileFormat = FileFormat {
    id: 105_857_104,
    puid: "wikidata/105857104",
    name: "GNU Privacy Guard public keyring (generic)",
    extensions: &["gpg"],
    media_types: &["application/pgp-keys"],
    internal_signatures: &[],
    related_formats: &[],
};

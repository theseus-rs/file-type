use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_196765: FileFormat = FileFormat {
    id: 196_765,
    puid: "wikidata/196765",
    name: "revocation list",
    extensions: &["crl"],
    media_types: &["application/pkix-crl"],
    internal_signatures: &[],
    related_formats: &[],
};

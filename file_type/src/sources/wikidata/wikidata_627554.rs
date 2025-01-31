use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_627554: FileFormat = FileFormat {
    id: 627_554,
    puid: "wikidata/627554",
    name: "certificate signing request",
    extensions: &["csr", "p10"],
    media_types: &["application/pkcs10", "application/pkcs10"],
    internal_signatures: &[],
    related_formats: &[],
};

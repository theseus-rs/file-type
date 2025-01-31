use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_904873: FileFormat = FileFormat {
    id: 904_873,
    puid: "wikidata/904873",
    name: "Cryptographic Message Syntax",
    extensions: &["cmsc"],
    media_types: &["application/cms"],
    internal_signatures: &[],
    related_formats: &[],
};

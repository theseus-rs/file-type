use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116375924: FileFormat = FileFormat {
    id: 116_375_924,
    puid: "wikidata/116375924",
    name: "Access Database (2003 and earlier)",
    extensions: &["mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

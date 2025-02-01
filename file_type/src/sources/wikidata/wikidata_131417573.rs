use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131417573: FileFormat = FileFormat {
    id: 131_417_573,
    puid: "wikidata/131417573",
    name: "FRACT file",
    extensions: &["FRACT"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};

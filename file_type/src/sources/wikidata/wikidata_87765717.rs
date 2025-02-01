use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87765717: FileFormat = FileFormat {
    id: 87_765_717,
    puid: "wikidata/87765717",
    name: "EIOffice Document",
    extensions: &["eio"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

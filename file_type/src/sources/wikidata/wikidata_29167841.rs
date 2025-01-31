use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167841: FileFormat = FileFormat {
    id: 29_167_841,
    puid: "wikidata/29167841",
    name: "Outerra",
    extensions: &["otx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

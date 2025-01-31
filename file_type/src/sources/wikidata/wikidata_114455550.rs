use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114455550: FileFormat = FileFormat {
    id: 114_455_550,
    puid: "wikidata/114455550",
    name: "Apache Avro IDL Data",
    extensions: &["avdl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

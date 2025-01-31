use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6108932: FileFormat = FileFormat {
    id: 6_108_932,
    puid: "wikidata/6108932",
    name: "JSGF",
    extensions: &["jsgf", "jsgf", "jsgf"],
    media_types: &["application/jsgf", "application/x-jsgf", "text/jsgf"],
    internal_signatures: &[],
    related_formats: &[],
};

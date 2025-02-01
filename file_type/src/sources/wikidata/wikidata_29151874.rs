use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29151874: FileFormat = FileFormat {
    id: 29_151_874,
    puid: "wikidata/29151874",
    name: "QRT Ray Tracer scene description",
    extensions: &["qrt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

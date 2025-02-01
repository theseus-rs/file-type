use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118145066: FileFormat = FileFormat {
    id: 118_145_066,
    puid: "wikidata/118145066",
    name: "Serenade Harmonica File",
    extensions: &["ckt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

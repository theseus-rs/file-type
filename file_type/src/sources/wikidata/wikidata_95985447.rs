use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_95985447: FileFormat = FileFormat {
    id: 95_985_447,
    puid: "wikidata/95985447",
    name: "Affymetrix GIN file format",
    extensions: &["gin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

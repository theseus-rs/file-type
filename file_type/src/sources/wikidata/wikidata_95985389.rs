use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_95985389: FileFormat = FileFormat {
    id: 95_985_389,
    puid: "wikidata/95985389",
    name: "Affymetrix CHP file format",
    extensions: &["chp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

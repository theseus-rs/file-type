use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109682807: FileFormat = FileFormat {
    id: 109_682_807,
    puid: "wikidata/109682807",
    name: "Sinar Digital Back format",
    extensions: &["sti"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

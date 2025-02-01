use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2276274: FileFormat = FileFormat {
    id: 2_276_274,
    puid: "wikidata/2276274",
    name: "System.map",
    extensions: &["map"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

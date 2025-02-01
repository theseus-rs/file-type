use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118976922: FileFormat = FileFormat {
    id: 118_976_922,
    puid: "wikidata/118976922",
    name: "FreeHand Template 11",
    extensions: &["ft11"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

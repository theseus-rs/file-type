use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29651047: FileFormat = FileFormat {
    id: 29_651_047,
    puid: "wikidata/29651047",
    name: "PaperPort",
    extensions: &["max"],
    media_types: &["application/x-paperport"],
    internal_signatures: &[],
    related_formats: &[],
};

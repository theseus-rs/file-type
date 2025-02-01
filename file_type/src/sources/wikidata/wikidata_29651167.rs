use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29651167: FileFormat = FileFormat {
    id: 29_651_167,
    puid: "wikidata/29651167",
    name: "Personal Address Book",
    extensions: &["pab"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

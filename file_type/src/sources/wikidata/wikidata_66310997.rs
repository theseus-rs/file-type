use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66310997: FileFormat = FileFormat {
    id: 66_310_997,
    puid: "wikidata/66310997",
    name: "Favorite Files",
    extensions: &["mfv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

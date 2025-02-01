use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29151939: FileFormat = FileFormat {
    id: 29_151_939,
    puid: "wikidata/29151939",
    name: "Random Dot Software Graphic QDV",
    extensions: &["qdv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

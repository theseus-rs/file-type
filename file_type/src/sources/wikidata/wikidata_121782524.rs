use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121782524: FileFormat = FileFormat {
    id: 121_782_524,
    puid: "wikidata/121782524",
    name: "Adobe Illustrator CC Artwork 17-23",
    extensions: &["ai", "ait"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

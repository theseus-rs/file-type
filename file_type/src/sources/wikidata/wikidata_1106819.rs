use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1106819: FileFormat = FileFormat {
    id: 1_106_819,
    puid: "wikidata/1106819",
    name: "CoffeeScript",
    extensions: &["coffee", "coffee"],
    media_types: &["application/vnd.coffeescript", "text/coffeescript"],
    internal_signatures: &[],
    related_formats: &[],
};

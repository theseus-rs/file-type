use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1106819: FileFormat = FileFormat {
    id: 1_106_819,
    source_type: SourceType::Wikidata,
    name: "CoffeeScript",
    extensions: &["coffee"],
    media_types: &["application/vnd.coffeescript", "text/coffeescript"],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1106819: FileFormat = FileFormat {
    id: 1_106_819,
    source_type: SourceType::Wikidata,
    name: "CoffeeScript",
    extensions: &["coffee"],
    media_types: &["application/vnd.coffeescript", "text/coffeescript"],
    internal_signatures: &[],
    related_formats: &[],
};

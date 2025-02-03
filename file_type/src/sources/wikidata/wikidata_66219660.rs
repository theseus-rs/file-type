use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66219660: FileFormat = FileFormat {
    id: 66_219_660,
    source_type: SourceType::Wikidata,
    name: "shtml",
    extensions: &["sht", "shtm", "shtml", "stm"],
    media_types: &["text/x-server-parsed-html", "text/x-server-parsed-html3"],
    internal_signatures: &[],
    related_formats: &[],
};

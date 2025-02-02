use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126811698: FileFormat = FileFormat {
    id: 126_811_698,
    source_type: SourceType::Wikidata,
    name: "Booasm Compressed Archive",
    extensions: &["boo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

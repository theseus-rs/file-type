use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850920: FileFormat = FileFormat {
    id: 105_850_920,
    source_type: SourceType::Wikidata,
    name: "Apache Tapestry Markup Language document",
    extensions: &["tml"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};

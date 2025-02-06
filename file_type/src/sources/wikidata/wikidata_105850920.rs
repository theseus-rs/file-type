use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850920: FileFormat = FileFormat {
    id: 105_850_920,
    source_type: SourceType::Wikidata,
    name: "Apache Tapestry Markup Language document",
    extensions: &["tml"],
    media_types: &["text/html"],
    signatures: &[],
    related_formats: &[],
};

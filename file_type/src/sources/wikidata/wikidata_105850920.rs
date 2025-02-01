use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850920: FileFormat = FileFormat {
    id: 105_850_920,
    puid: "wikidata/105850920",
    name: "Apache Tapestry Markup Language document",
    extensions: &["tml"],
    media_types: &["text/html"],
    internal_signatures: &[],
    related_formats: &[],
};

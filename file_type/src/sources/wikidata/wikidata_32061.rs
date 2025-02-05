use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_32061: FileFormat = FileFormat {
    id: 32_061,
    source_type: SourceType::Wikidata,
    name: "XSL",
    extensions: &["xsl", "xslt"],
    media_types: &["application/xslt+xml"],
    signatures: &[],
    related_formats: &[],
};

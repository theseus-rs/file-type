use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_32061: FileFormat = FileFormat {
    id: 32_061,
    source_type: SourceType::Wikidata,
    name: "XSL",
    extensions: &["xsl", "xslt"],
    media_types: &["application/xslt+xml"],
    internal_signatures: &[],
    related_formats: &[],
};

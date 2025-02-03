use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109285453: FileFormat = FileFormat {
    id: 109_285_453,
    source_type: SourceType::Wikidata,
    name: "phtml",
    extensions: &["phtml"],
    media_types: &["application/x-httpd-php"],
    internal_signatures: &[],
    related_formats: &[],
};

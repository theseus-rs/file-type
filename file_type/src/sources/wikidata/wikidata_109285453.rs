use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109285453: FileFormat = FileFormat {
    id: 109_285_453,
    source_type: SourceType::Wikidata,
    name: "phtml",
    extensions: &["phtml"],
    media_types: &["application/x-httpd-php"],
    signatures: &[],
    related_formats: &[],
};

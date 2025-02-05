use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51837307: FileFormat = FileFormat {
    id: 51_837_307,
    source_type: SourceType::Wikidata,
    name: "IBM DisplayWrite DCA Text File",
    extensions: &["dca"],
    media_types: &["application/dca-rft"],
    signatures: &[],
    related_formats: &[],
};

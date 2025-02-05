use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120716854: FileFormat = FileFormat {
    id: 120_716_854,
    source_type: SourceType::Wikidata,
    name: "TaxCut 2006 Tax Return file",
    extensions: &["t06"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

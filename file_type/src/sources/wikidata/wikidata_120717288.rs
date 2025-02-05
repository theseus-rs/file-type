use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120717288: FileFormat = FileFormat {
    id: 120_717_288,
    source_type: SourceType::Wikidata,
    name: "TaxCut 2007 Tax Return file",
    extensions: &["t07"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

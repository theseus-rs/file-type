use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120966179: FileFormat = FileFormat {
    id: 120_966_179,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money 98 data file",
    extensions: &["mn6"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120920814: FileFormat = FileFormat {
    id: 120_920_814,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money 2006 data",
    extensions: &["m15"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120920280: FileFormat = FileFormat {
    id: 120_920_280,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money 2002 data",
    extensions: &["m10"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

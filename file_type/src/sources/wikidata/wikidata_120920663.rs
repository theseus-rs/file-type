use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120920663: FileFormat = FileFormat {
    id: 120_920_663,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money 2003 data",
    extensions: &["m11"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

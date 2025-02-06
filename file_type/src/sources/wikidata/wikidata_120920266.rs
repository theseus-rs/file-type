use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120920266: FileFormat = FileFormat {
    id: 120_920_266,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money version 1 data",
    extensions: &["mn1"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55753012: FileFormat = FileFormat {
    id: 55_753_012,
    source_type: SourceType::Wikidata,
    name: "Microsoft xWMA file format",
    extensions: &["xwma"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

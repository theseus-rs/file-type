use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120966262: FileFormat = FileFormat {
    id: 120_966_262,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money 99 data",
    extensions: &["mn7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

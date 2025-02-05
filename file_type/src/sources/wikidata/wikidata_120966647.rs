use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120966647: FileFormat = FileFormat {
    id: 120_966_647,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money 2000 data file",
    extensions: &["mn8"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

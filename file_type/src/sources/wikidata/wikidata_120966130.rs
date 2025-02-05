use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120966130: FileFormat = FileFormat {
    id: 120_966_130,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money 97 data",
    extensions: &["mn5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

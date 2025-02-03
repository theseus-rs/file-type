use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120966130: FileFormat = FileFormat {
    id: 120_966_130,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money 97 data",
    extensions: &["mn5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

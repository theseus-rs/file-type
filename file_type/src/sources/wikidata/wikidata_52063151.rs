use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_52063151: FileFormat = FileFormat {
    id: 52_063_151,
    source_type: SourceType::Wikidata,
    name: "Lotus Notes File",
    extensions: &["box"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

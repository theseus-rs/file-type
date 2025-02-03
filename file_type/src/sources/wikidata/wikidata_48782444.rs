use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48782444: FileFormat = FileFormat {
    id: 48_782_444,
    source_type: SourceType::Wikidata,
    name: "ACBM Graphics",
    extensions: &["acb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

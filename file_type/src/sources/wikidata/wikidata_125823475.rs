use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125823475: FileFormat = FileFormat {
    id: 125_823_475,
    source_type: SourceType::Wikidata,
    name: "Microsoft Help merged query index file",
    extensions: &["hxq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

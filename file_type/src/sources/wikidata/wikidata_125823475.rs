use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125823475: FileFormat = FileFormat {
    id: 125_823_475,
    source_type: SourceType::Wikidata,
    name: "Microsoft Help merged query index file",
    extensions: &["hxq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

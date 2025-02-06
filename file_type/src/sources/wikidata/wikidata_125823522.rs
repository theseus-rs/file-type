use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125823522: FileFormat = FileFormat {
    id: 125_823_522,
    source_type: SourceType::Wikidata,
    name: "Microsoft Help Attribute Definition file",
    extensions: &["hxw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

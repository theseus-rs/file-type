use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125823522: FileFormat = FileFormat {
    id: 125_823_522,
    source_type: SourceType::Wikidata,
    name: "Microsoft Help Attribute Definition file",
    extensions: &["hxw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

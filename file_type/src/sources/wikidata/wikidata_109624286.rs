use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109624286: FileFormat = FileFormat {
    id: 109_624_286,
    source_type: SourceType::Wikidata,
    name: "WebPlus Essentials Site",
    extensions: &["wpp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

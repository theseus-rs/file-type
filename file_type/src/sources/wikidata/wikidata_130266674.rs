use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130266674: FileFormat = FileFormat {
    id: 130_266_674,
    source_type: SourceType::Wikidata,
    name: "Luau source code file",
    extensions: &["luau"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

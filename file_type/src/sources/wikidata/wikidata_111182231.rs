use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111182231: FileFormat = FileFormat {
    id: 111_182_231,
    source_type: SourceType::Wikidata,
    name: "ActionScript Communication File",
    extensions: &["asc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

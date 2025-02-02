use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111391892: FileFormat = FileFormat {
    id: 111_391_892,
    source_type: SourceType::Wikidata,
    name: "Bryce2 Object",
    extensions: &["obj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

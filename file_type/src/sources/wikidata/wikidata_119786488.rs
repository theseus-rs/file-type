use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119786488: FileFormat = FileFormat {
    id: 119_786_488,
    source_type: SourceType::Wikidata,
    name: "MasterCook Export File",
    extensions: &["mx2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

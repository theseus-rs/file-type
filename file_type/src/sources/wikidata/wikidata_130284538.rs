use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130284538: FileFormat = FileFormat {
    id: 130_284_538,
    source_type: SourceType::Wikidata,
    name: "MCfunction script file",
    extensions: &["mcfunction"],
    media_types: &["text/mcfunction"],
    internal_signatures: &[],
    related_formats: &[],
};

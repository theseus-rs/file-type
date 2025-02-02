use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979410: FileFormat = FileFormat {
    id: 27_979_410,
    source_type: SourceType::Wikidata,
    name: "Binary Text",
    extensions: &["bin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

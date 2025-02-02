use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128693639: FileFormat = FileFormat {
    id: 128_693_639,
    source_type: SourceType::Wikidata,
    name: "BBC Basic file",
    extensions: &["bbc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128693639: FileFormat = FileFormat {
    id: 128_693_639,
    source_type: SourceType::Wikidata,
    name: "BBC Basic file",
    extensions: &["bbc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128693639: FileFormat = FileFormat {
    id: 128_693_639,
    puid: "wikidata/128693639",
    name: "BBC Basic file",
    extensions: &["bbc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979410: FileFormat = FileFormat {
    id: 27_979_410,
    source_type: SourceType::Wikidata,
    name: "Binary Text",
    extensions: &["bin"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

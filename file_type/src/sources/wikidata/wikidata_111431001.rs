use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111431001: FileFormat = FileFormat {
    id: 111_431_001,
    source_type: SourceType::Wikidata,
    name: "ExtendScript Included Script File",
    extensions: &["jsxinc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

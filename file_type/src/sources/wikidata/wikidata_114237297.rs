use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114237297: FileFormat = FileFormat {
    id: 114_237_297,
    source_type: SourceType::Wikidata,
    name: "Visual C++ Project file",
    extensions: &["mak"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

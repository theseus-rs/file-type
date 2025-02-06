use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122673484: FileFormat = FileFormat {
    id: 122_673_484,
    source_type: SourceType::Wikidata,
    name: "Outline 4D Document",
    extensions: &["syv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

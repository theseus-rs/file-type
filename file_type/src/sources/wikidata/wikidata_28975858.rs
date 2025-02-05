use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975858: FileFormat = FileFormat {
    id: 28_975_858,
    source_type: SourceType::Wikidata,
    name: "OOGL QUAD file",
    extensions: &["quad"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

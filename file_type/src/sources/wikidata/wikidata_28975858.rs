use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975858: FileFormat = FileFormat {
    id: 28_975_858,
    source_type: SourceType::Wikidata,
    name: "OOGL QUAD file",
    extensions: &["quad"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

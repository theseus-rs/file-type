use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855797: FileFormat = FileFormat {
    id: 105_855_797,
    source_type: SourceType::Wikidata,
    name: "Vivid DiffSet",
    extensions: &["dsx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855578: FileFormat = FileFormat {
    id: 105_855_578,
    source_type: SourceType::Wikidata,
    name: "OpenShot effect",
    extensions: &["xml"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};

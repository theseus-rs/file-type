use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27959791: FileFormat = FileFormat {
    id: 27_959_791,
    source_type: SourceType::Wikidata,
    name: "Ableton Device Group",
    extensions: &["adg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

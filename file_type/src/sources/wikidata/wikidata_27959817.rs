use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27959817: FileFormat = FileFormat {
    id: 27_959_817,
    source_type: SourceType::Wikidata,
    name: "Ableton Max for Live Device",
    extensions: &["amxd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

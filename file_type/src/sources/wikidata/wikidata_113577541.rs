use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113577541: FileFormat = FileFormat {
    id: 113_577_541,
    source_type: SourceType::Wikidata,
    name: "DiscJuggler Image",
    extensions: &["cdi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113577541: FileFormat = FileFormat {
    id: 113_577_541,
    source_type: SourceType::Wikidata,
    name: "DiscJuggler Image",
    extensions: &["cdi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

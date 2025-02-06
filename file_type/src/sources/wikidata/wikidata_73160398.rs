use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73160398: FileFormat = FileFormat {
    id: 73_160_398,
    source_type: SourceType::Wikidata,
    name: "Visio Stencil",
    extensions: &["vss"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

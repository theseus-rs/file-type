use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73160398: FileFormat = FileFormat {
    id: 73_160_398,
    puid: "wikidata/73160398",
    name: "Visio Stencil",
    extensions: &["vss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

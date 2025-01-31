use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119966628: FileFormat = FileFormat {
    id: 119_966_628,
    puid: "wikidata/119966628",
    name: "Pocket Streets Map",
    extensions: &["mps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

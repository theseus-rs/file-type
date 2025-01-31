use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119097093: FileFormat = FileFormat {
    id: 119_097_093,
    puid: "wikidata/119097093",
    name: "Fireworks PNG",
    extensions: &["png"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

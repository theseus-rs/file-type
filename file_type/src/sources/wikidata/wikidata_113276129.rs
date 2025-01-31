use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113276129: FileFormat = FileFormat {
    id: 113_276_129,
    puid: "wikidata/113276129",
    name: "The Print Shop Deluxe Photo Pages",
    extensions: &["pho"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

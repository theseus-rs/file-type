use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87121992: FileFormat = FileFormat {
    id: 87_121_992,
    puid: "wikidata/87121992",
    name: "Open Financial Exchange 2.0.3",
    extensions: &["ofx", "qfx"],
    media_types: &["application/x-ofx", "application/x-ofx"],
    internal_signatures: &[],
    related_formats: &[],
};

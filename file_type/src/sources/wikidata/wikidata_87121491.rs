use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87121491: FileFormat = FileFormat {
    id: 87_121_491,
    puid: "wikidata/87121491",
    name: "Open Financial Exchange 1.6",
    extensions: &["ofx", "qfx"],
    media_types: &["application/x-ofx", "application/x-ofx"],
    internal_signatures: &[],
    related_formats: &[],
};

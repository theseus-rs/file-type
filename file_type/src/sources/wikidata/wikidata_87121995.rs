use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87121995: FileFormat = FileFormat {
    id: 87_121_995,
    puid: "wikidata/87121995",
    name: "Open Financial Exchange 2.1.1",
    extensions: &["ofx", "qfx"],
    media_types: &["application/x-ofx", "application/x-ofx"],
    internal_signatures: &[],
    related_formats: &[],
};

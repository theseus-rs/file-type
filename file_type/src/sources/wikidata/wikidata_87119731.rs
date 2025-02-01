use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87119731: FileFormat = FileFormat {
    id: 87_119_731,
    puid: "wikidata/87119731",
    name: "Open Financial Exchange 1.02",
    extensions: &["ofx", "qfx"],
    media_types: &["application/x-ofx", "application/x-ofx"],
    internal_signatures: &[],
    related_formats: &[],
};

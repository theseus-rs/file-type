use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87119735: FileFormat = FileFormat {
    id: 87_119_735,
    puid: "wikidata/87119735",
    name: "Open Financial Exchange 1.03",
    extensions: &["ofx", "qfx"],
    media_types: &["application/x-ofx", "application/x-ofx"],
    internal_signatures: &[],
    related_formats: &[],
};

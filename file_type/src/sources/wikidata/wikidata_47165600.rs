use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47165600: FileFormat = FileFormat {
    id: 47_165_600,
    puid: "wikidata/47165600",
    name: "RealLegal E-Transcript file format",
    extensions: &["ptx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

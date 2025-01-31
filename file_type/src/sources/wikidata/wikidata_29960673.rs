use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29960673: FileFormat = FileFormat {
    id: 29_960_673,
    puid: "wikidata/29960673",
    name: "Avantes USB spectrometer ROH file",
    extensions: &["roh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47462143: FileFormat = FileFormat {
    id: 47_462_143,
    puid: "wikidata/47462143",
    name: "Caligari TrueSpace file format",
    extensions: &["cob", "scn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

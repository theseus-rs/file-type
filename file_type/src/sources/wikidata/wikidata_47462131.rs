use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47462131: FileFormat = FileFormat {
    id: 47_462_131,
    puid: "wikidata/47462131",
    name: "Caligari TrueSpace file format (ASCII)",
    extensions: &["cob", "scn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

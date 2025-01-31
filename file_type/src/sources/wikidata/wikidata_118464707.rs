use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118464707: FileFormat = FileFormat {
    id: 118_464_707,
    puid: "wikidata/118464707",
    name: "Open Media Framework Interchange 1.0",
    extensions: &["omf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

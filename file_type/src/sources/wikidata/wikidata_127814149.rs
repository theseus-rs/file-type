use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127814149: FileFormat = FileFormat {
    id: 127_814_149,
    puid: "wikidata/127814149",
    name: "Ox source code file",
    extensions: &["ox"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

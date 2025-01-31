use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127411070: FileFormat = FileFormat {
    id: 127_411_070,
    puid: "wikidata/127411070",
    name: "Nim source code file",
    extensions: &["nim", "nimrod"],
    media_types: &["text/x-nim", "text/x-nim"],
    internal_signatures: &[],
    related_formats: &[],
};

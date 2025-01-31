use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130736665: FileFormat = FileFormat {
    id: 130_736_665,
    puid: "wikidata/130736665",
    name: "Savi source code file",
    extensions: &["savi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_76622680: FileFormat = FileFormat {
    id: 76_622_680,
    puid: "wikidata/76622680",
    name: "Turboprint Wizard",
    extensions: &["wizard"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

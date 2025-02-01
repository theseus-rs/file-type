use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206351: FileFormat = FileFormat {
    id: 28_206_351,
    puid: "wikidata/28206351",
    name: "Inset PIX",
    extensions: &["pix"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

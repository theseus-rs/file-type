use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_72274847: FileFormat = FileFormat {
    id: 72_274_847,
    puid: "wikidata/72274847",
    name: "Maestro molecular model",
    extensions: &["mae"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

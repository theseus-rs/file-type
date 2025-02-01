use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600495: FileFormat = FileFormat {
    id: 28_600_495,
    puid: "wikidata/28600495",
    name: "Dia",
    extensions: &["dia"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205381: FileFormat = FileFormat {
    id: 28_205_381,
    puid: "wikidata/28205381",
    name: "Lytro",
    extensions: &["lfp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

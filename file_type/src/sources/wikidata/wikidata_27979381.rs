use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979381: FileFormat = FileFormat {
    id: 27_979_381,
    puid: "wikidata/27979381",
    name: "Blu-ray Clip info",
    extensions: &["clp", "clpi", "cpi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

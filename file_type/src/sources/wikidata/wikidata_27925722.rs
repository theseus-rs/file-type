use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27925722: FileFormat = FileFormat {
    id: 27_925_722,
    puid: "wikidata/27925722",
    name: "DTED Level 1 Gazetteer Directory file",
    extensions: &["dir"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

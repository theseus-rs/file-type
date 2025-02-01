use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111263049: FileFormat = FileFormat {
    id: 111_263_049,
    puid: "wikidata/111263049",
    name: "ActiveMovie streaming format",
    extensions: &["asf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

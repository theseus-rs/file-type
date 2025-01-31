use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128203989: FileFormat = FileFormat {
    id: 128_203_989,
    puid: "wikidata/128203989",
    name: "TorqueScript file",
    extensions: &["cs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

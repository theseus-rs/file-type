use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49989019: FileFormat = FileFormat {
    id: 49_989_019,
    puid: "wikidata/49989019",
    name: "Macromedia (Adobe) Director Compressed Resource file",
    extensions: &["dcr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

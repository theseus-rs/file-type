use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862407: FileFormat = FileFormat {
    id: 105_862_407,
    puid: "wikidata/105862407",
    name: "Mac Compact Pro archive",
    extensions: &["cpt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975870: FileFormat = FileFormat {
    id: 28_975_870,
    puid: "wikidata/28975870",
    name: "OOGL INST file",
    extensions: &["inst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

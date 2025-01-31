use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975669: FileFormat = FileFormat {
    id: 28_975_669,
    puid: "wikidata/28975669",
    name: "BMF",
    extensions: &["bmf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59716162: FileFormat = FileFormat {
    id: 59_716_162,
    puid: "wikidata/59716162",
    name: "Harvard Graphics Chart",
    extensions: &["ch3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

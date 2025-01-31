use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116808623: FileFormat = FileFormat {
    id: 116_808_623,
    puid: "wikidata/116808623",
    name: "WillMaker File",
    extensions: &["ww8"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

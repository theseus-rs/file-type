use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118464834: FileFormat = FileFormat {
    id: 118_464_834,
    puid: "wikidata/118464834",
    name: "Enhanced Image Package",
    extensions: &["eip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

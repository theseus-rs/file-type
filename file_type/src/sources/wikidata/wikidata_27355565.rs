use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27355565: FileFormat = FileFormat {
    id: 27_355_565,
    puid: "wikidata/27355565",
    name: "ADRG Quality File",
    extensions: &["qal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

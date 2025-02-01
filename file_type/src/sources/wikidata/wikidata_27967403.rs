use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967403: FileFormat = FileFormat {
    id: 27_967_403,
    puid: "wikidata/27967403",
    name: "CUD-FM-File",
    extensions: &["cff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979401: FileFormat = FileFormat {
    id: 27_979_401,
    puid: "wikidata/27979401",
    name: "JP2",
    extensions: &["jp2"],
    media_types: &["image/jp2"],
    internal_signatures: &[],
    related_formats: &[],
};

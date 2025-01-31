use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131341835: FileFormat = FileFormat {
    id: 131_341_835,
    puid: "wikidata/131341835",
    name: "UrbiScript source code file",
    extensions: &["u"],
    media_types: &["application/x-urbiscript"],
    internal_signatures: &[],
    related_formats: &[],
};

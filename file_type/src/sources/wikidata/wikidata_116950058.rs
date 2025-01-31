use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116950058: FileFormat = FileFormat {
    id: 116_950_058,
    puid: "wikidata/116950058",
    name: "Ulead COOL 360 Project File",
    extensions: &["upj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

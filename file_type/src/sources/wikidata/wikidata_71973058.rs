use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71973058: FileFormat = FileFormat {
    id: 71_973_058,
    puid: "wikidata/71973058",
    name: "CorelDraw Drawing, version X5",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

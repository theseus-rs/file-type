use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959791: FileFormat = FileFormat {
    id: 27_959_791,
    puid: "wikidata/27959791",
    name: "Ableton Device Group",
    extensions: &["adg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

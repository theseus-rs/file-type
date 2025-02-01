use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27894870: FileFormat = FileFormat {
    id: 27_894_870,
    puid: "wikidata/27894870",
    name: "Windows Media Audio Redirector",
    extensions: &["wax"],
    media_types: &["audio/x-ms-wax"],
    internal_signatures: &[],
    related_formats: &[],
};

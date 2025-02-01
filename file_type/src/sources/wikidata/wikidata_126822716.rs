use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126822716: FileFormat = FileFormat {
    id: 126_822_716,
    puid: "wikidata/126822716",
    name: "Visual F# Source File",
    extensions: &["fs"],
    media_types: &["text/x-fsharp"],
    internal_signatures: &[],
    related_formats: &[],
};

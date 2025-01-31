use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126951498: FileFormat = FileFormat {
    id: 126_951_498,
    puid: "wikidata/126951498",
    name: "Haxe source code file",
    extensions: &["hx", "hx", "hx"],
    media_types: &["text/haxe", "text/x-haxe", "text/x-hx"],
    internal_signatures: &[],
    related_formats: &[],
};

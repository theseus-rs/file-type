use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126951861: FileFormat = FileFormat {
    id: 126_951_861,
    puid: "wikidata/126951861",
    name: "Scala source code file",
    extensions: &["scala"],
    media_types: &["text/x-scala"],
    internal_signatures: &[],
    related_formats: &[],
};

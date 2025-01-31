use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126951686: FileFormat = FileFormat {
    id: 126_951_686,
    puid: "wikidata/126951686",
    name: "Nemerle source code file",
    extensions: &["n"],
    media_types: &["text/x-nemerle"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130057181: FileFormat = FileFormat {
    id: 130_057_181,
    puid: "wikidata/130057181",
    name: "K source code file",
    extensions: &["k"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

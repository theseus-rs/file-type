use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130548424: FileFormat = FileFormat {
    id: 130_548_424,
    puid: "wikidata/130548424",
    name: "QBasic source code file",
    extensions: &["bas"],
    media_types: &["text/basic"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49243071: FileFormat = FileFormat {
    id: 49_243_071,
    puid: "wikidata/49243071",
    name: "Java language source code file",
    extensions: &["java"],
    media_types: &["text/x-java"],
    internal_signatures: &[],
    related_formats: &[],
};

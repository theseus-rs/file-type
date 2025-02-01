use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71178742: FileFormat = FileFormat {
    id: 71_178_742,
    puid: "wikidata/71178742",
    name: "PIPE2 file format",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

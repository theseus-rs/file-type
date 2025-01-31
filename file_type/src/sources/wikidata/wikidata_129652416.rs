use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129652416: FileFormat = FileFormat {
    id: 129_652_416,
    puid: "wikidata/129652416",
    name: "Inform 6 template file",
    extensions: &["i6t"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

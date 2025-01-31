use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129167999: FileFormat = FileFormat {
    id: 129_167_999,
    puid: "wikidata/129167999",
    name: "Factor source code file",
    extensions: &["factor"],
    media_types: &["text/x-factor"],
    internal_signatures: &[],
    related_formats: &[],
};

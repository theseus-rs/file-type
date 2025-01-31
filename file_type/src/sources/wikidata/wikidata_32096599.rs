use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_32096599: FileFormat = FileFormat {
    id: 32_096_599,
    puid: "wikidata/32096599",
    name: "Gran Turismo File System",
    extensions: &["vol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

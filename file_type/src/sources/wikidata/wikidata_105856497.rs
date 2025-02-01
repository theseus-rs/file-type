use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856497: FileFormat = FileFormat {
    id: 105_856_497,
    puid: "wikidata/105856497",
    name: "Khoros Visual Programming Workspace (with rem)",
    extensions: &["wk"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};

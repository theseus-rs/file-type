use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130396153: FileFormat = FileFormat {
    id: 130_396_153,
    puid: "wikidata/130396153",
    name: "Ooc source code file",
    extensions: &["ooc"],
    media_types: &["text/x-ooc"],
    internal_signatures: &[],
    related_formats: &[],
};

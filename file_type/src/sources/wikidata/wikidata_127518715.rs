use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127518715: FileFormat = FileFormat {
    id: 127_518_715,
    puid: "wikidata/127518715",
    name: "Zephir source code file",
    extensions: &["zep"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

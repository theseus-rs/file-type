use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124097900: FileFormat = FileFormat {
    id: 124_097_900,
    puid: "wikidata/124097900",
    name: ".txt file",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

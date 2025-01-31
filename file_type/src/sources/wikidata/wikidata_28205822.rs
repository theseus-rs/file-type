use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205822: FileFormat = FileFormat {
    id: 28_205_822,
    puid: "wikidata/28205822",
    name: "CD5",
    extensions: &["cd5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

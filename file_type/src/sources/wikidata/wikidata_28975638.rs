use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975638: FileFormat = FileFormat {
    id: 28_975_638,
    puid: "wikidata/28975638",
    name: "Parasolid",
    extensions: &["x_t"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

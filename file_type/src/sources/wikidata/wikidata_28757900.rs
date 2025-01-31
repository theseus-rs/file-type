use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757900: FileFormat = FileFormat {
    id: 28_757_900,
    puid: "wikidata/28757900",
    name: "Glyph Interchange Format",
    extensions: &["glif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

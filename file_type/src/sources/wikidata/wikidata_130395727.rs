use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130395727: FileFormat = FileFormat {
    id: 130_395_727,
    puid: "wikidata/130395727",
    name: "ODIN file format",
    extensions: &["odin"],
    media_types: &["text/odin"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_77045990: FileFormat = FileFormat {
    id: 77_045_990,
    puid: "wikidata/77045990",
    name: "Extensible 3D vector graphics (binary)",
    extensions: &["x3db"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

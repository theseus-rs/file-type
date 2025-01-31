use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130267688: FileFormat = FileFormat {
    id: 130_267_688,
    puid: "wikidata/130267688",
    name: "STL file format",
    extensions: &["stl"],
    media_types: &["model/stl"],
    internal_signatures: &[],
    related_formats: &[],
};

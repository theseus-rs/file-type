use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117035605: FileFormat = FileFormat {
    id: 117_035_605,
    puid: "wikidata/117035605",
    name: "VRML geography data",
    extensions: &["geo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

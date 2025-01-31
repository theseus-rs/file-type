use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27925699: FileFormat = FileFormat {
    id: 27_925_699,
    puid: "wikidata/27925699",
    name: "DTED Level 0 Average Terrain Elevation Value File",
    extensions: &["avg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

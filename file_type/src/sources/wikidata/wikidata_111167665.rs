use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111167665: FileFormat = FileFormat {
    id: 111_167_665,
    puid: "wikidata/111167665",
    name: "ChemSketch 1.0 file",
    extensions: &["mst", "rpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

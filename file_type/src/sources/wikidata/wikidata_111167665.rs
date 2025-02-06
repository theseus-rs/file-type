use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111167665: FileFormat = FileFormat {
    id: 111_167_665,
    source_type: SourceType::Wikidata,
    name: "ChemSketch 1.0 file",
    extensions: &["mst", "rpt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

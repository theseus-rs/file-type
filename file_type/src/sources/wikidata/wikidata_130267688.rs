use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130267688: FileFormat = FileFormat {
    id: 130_267_688,
    source_type: SourceType::Wikidata,
    name: "STL file format",
    extensions: &["stl"],
    media_types: &["model/stl"],
    signatures: &[],
    related_formats: &[],
};

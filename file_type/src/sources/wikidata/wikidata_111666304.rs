use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111666304: FileFormat = FileFormat {
    id: 111_666_304,
    source_type: SourceType::Wikidata,
    name: "Liveart Sketches",
    extensions: &["lrt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

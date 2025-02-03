use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111666304: FileFormat = FileFormat {
    id: 111_666_304,
    source_type: SourceType::Wikidata,
    name: "Liveart Sketches",
    extensions: &["lrt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

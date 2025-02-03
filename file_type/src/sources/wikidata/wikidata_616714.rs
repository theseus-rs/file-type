use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_616714: FileFormat = FileFormat {
    id: 616_714,
    source_type: SourceType::Wikidata,
    name: "Initial Graphics Exchange Specification",
    extensions: &["iges", "igs"],
    media_types: &["application/iges", "model/iges"],
    internal_signatures: &[],
    related_formats: &[],
};

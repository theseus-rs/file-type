use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_616714: FileFormat = FileFormat {
    id: 616_714,
    source_type: SourceType::Wikidata,
    name: "Initial Graphics Exchange Specification",
    extensions: &["iges", "igs"],
    media_types: &["application/iges", "model/iges"],
    signatures: &[],
    related_formats: &[],
};

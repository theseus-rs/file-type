use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111432169: FileFormat = FileFormat {
    id: 111_432_169,
    source_type: SourceType::Wikidata,
    name: "Hypertext Template",
    extensions: &["htt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

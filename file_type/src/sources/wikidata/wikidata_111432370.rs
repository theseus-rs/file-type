use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111432370: FileFormat = FileFormat {
    id: 111_432_370,
    source_type: SourceType::Wikidata,
    name: "Interface Definition Language File",
    extensions: &["idl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47165998: FileFormat = FileFormat {
    id: 47_165_998,
    source_type: SourceType::Wikidata,
    name: "ClarisWorks file format version 1",
    extensions: &["cwk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

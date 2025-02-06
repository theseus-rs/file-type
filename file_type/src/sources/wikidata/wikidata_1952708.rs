use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1952708: FileFormat = FileFormat {
    id: 1_952_708,
    source_type: SourceType::Wikidata,
    name: "FILE_ID.DIZ",
    extensions: &["diz"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};

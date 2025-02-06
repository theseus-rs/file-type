use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128775907: FileFormat = FileFormat {
    id: 128_775_907,
    source_type: SourceType::Wikidata,
    name: "Coq file format",
    extensions: &["v"],
    media_types: &["text/x-coq"],
    signatures: &[],
    related_formats: &[],
};

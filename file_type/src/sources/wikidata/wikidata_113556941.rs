use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113556941: FileFormat = FileFormat {
    id: 113_556_941,
    source_type: SourceType::Wikidata,
    name: "CDR-Win Image",
    extensions: &["bin"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

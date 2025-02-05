use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113577674: FileFormat = FileFormat {
    id: 113_577_674,
    source_type: SourceType::Wikidata,
    name: "Prassi PrimoDVD",
    extensions: &["gi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

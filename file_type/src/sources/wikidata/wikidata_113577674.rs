use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113577674: FileFormat = FileFormat {
    id: 113_577_674,
    source_type: SourceType::Wikidata,
    name: "Prassi PrimoDVD",
    extensions: &["gi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

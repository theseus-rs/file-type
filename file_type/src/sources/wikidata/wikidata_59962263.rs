use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59962263: FileFormat = FileFormat {
    id: 59_962_263,
    source_type: SourceType::Wikidata,
    name: "ChiWriter Document",
    extensions: &["chi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

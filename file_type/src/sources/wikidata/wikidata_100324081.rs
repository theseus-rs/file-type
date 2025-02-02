use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100324081: FileFormat = FileFormat {
    id: 100_324_081,
    source_type: SourceType::Wikidata,
    name: "Corel Print House Document, version 2",
    extensions: &["cpd", "cph"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

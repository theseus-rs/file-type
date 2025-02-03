use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100324042: FileFormat = FileFormat {
    id: 100_324_042,
    source_type: SourceType::Wikidata,
    name: "Corel Print House Document, version 1",
    extensions: &["cpd", "cph"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100324042: FileFormat = FileFormat {
    id: 100_324_042,
    source_type: SourceType::Wikidata,
    name: "Corel Print House Document, version 1",
    extensions: &["cpd", "cph"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

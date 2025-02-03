use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100324136: FileFormat = FileFormat {
    id: 100_324_136,
    source_type: SourceType::Wikidata,
    name: "Corel Print House/Print Office Document, version 3",
    extensions: &["cpd", "cph", "cpo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

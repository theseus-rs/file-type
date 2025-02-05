use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100324136: FileFormat = FileFormat {
    id: 100_324_136,
    source_type: SourceType::Wikidata,
    name: "Corel Print House/Print Office Document, version 3",
    extensions: &["cpd", "cph", "cpo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

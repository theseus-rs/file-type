use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27861482: FileFormat = FileFormat {
    id: 27_861_482,
    source_type: SourceType::Wikidata,
    name: "Renoise Song, version 10",
    extensions: &["xrns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

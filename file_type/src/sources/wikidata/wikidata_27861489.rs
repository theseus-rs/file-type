use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27861489: FileFormat = FileFormat {
    id: 27_861_489,
    source_type: SourceType::Wikidata,
    name: "Renoise Song, version 22",
    extensions: &["xrns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

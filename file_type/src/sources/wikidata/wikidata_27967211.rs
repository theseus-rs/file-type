use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967211: FileFormat = FileFormat {
    id: 27_967_211,
    source_type: SourceType::Wikidata,
    name: "Pumatracker module",
    extensions: &["puma"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

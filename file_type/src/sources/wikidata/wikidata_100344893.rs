use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100344893: FileFormat = FileFormat {
    id: 100_344_893,
    source_type: SourceType::Wikidata,
    name: "Corel Photo House Image",
    extensions: &["cps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

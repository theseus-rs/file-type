use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29651047: FileFormat = FileFormat {
    id: 29_651_047,
    source_type: SourceType::Wikidata,
    name: "PaperPort",
    extensions: &["max"],
    media_types: &["application/x-paperport"],
    internal_signatures: &[],
    related_formats: &[],
};

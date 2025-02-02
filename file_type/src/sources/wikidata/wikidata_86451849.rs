use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_86451849: FileFormat = FileFormat {
    id: 86_451_849,
    source_type: SourceType::Wikidata,
    name: "RFFlow Chart 5",
    extensions: &["flo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

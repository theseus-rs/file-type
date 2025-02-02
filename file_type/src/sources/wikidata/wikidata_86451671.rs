use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_86451671: FileFormat = FileFormat {
    id: 86_451_671,
    source_type: SourceType::Wikidata,
    name: "RFFlow Chart 4",
    extensions: &["flo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

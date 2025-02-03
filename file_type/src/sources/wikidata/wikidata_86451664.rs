use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_86451664: FileFormat = FileFormat {
    id: 86_451_664,
    source_type: SourceType::Wikidata,
    name: "RFFlow Chart 3",
    extensions: &["flo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

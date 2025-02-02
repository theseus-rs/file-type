use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121093196: FileFormat = FileFormat {
    id: 121_093_196,
    source_type: SourceType::Wikidata,
    name: "Punch! Home Suite PRO file",
    extensions: &["pro"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

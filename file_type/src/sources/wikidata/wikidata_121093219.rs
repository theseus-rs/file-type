use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121093219: FileFormat = FileFormat {
    id: 121_093_219,
    source_type: SourceType::Wikidata,
    name: "Punch! Home Suite PSH file",
    extensions: &["psh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

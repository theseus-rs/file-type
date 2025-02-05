use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121093219: FileFormat = FileFormat {
    id: 121_093_219,
    source_type: SourceType::Wikidata,
    name: "Punch! Home Suite PSH file",
    extensions: &["psh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

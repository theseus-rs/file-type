use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121093196: FileFormat = FileFormat {
    id: 121_093_196,
    source_type: SourceType::Wikidata,
    name: "Punch! Home Suite PRO file",
    extensions: &["pro"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

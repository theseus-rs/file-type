use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128203989: FileFormat = FileFormat {
    id: 128_203_989,
    source_type: SourceType::Wikidata,
    name: "TorqueScript file",
    extensions: &["cs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

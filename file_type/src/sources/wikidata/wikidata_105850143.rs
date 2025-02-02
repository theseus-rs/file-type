use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850143: FileFormat = FileFormat {
    id: 105_850_143,
    source_type: SourceType::Wikidata,
    name: "ArtCAM post processor Configuration",
    extensions: &["con"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x45, 0x53, 0x43, 0x52, 0x49, 0x50, 0x54, 0x49, 0x4F, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

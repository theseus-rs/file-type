use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859570: FileFormat = FileFormat {
    id: 105_859_570,
    source_type: SourceType::Wikidata,
    name: "virt-viewer configuration",
    extensions: &["vv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x76, 0x69, 0x72, 0x74, 0x2D, 0x76, 0x69, 0x65, 0x77, 0x65, 0x72, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

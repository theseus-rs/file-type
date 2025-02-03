use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47245245: FileFormat = FileFormat {
    id: 47_245_245,
    source_type: SourceType::Wikidata,
    name: "Microsoft Network Monitor Packet Capture, version 1",
    extensions: &["cap"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x54, 0x53, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};

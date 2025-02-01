use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860269: FileFormat = FileFormat {
    id: 105_860_269,
    puid: "wikidata/105860269",
    name: "DRC Rules export",
    extensions: &["rul"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x52, 0x43, 0x20, 0x52, 0x75, 0x6C, 0x65, 0x73, 0x20, 0x45, 0x78, 0x70,
                    0x6F, 0x72, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x20,
                    0x50, 0x43, 0x42, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

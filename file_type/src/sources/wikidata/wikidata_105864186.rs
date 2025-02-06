use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864186: FileFormat = FileFormat {
    id: 105_864_186,
    source_type: SourceType::Wikidata,
    name: "PCB 3.0 Binary file",
    extensions: &["pcb"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x13, 0x50, 0x43, 0x42, 0x20, 0x33, 0x2E, 0x30, 0x20, 0x42, 0x69, 0x6E, 0x61,
                    0x72, 0x79, 0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

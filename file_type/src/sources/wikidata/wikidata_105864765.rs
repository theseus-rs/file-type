use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864765: FileFormat = FileFormat {
    id: 105_864_765,
    source_type: SourceType::Wikidata,
    name: "CADSTAR PCB",
    extensions: &["pcb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x43, 0x42, 0x0D, 0x0A, 0x44, 0x61, 0x74, 0x61, 0x20, 0x53, 0x74, 0x72,
                    0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x20, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73,
                    0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

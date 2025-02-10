use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857550: FileFormat = FileFormat {
    id: 105_857_550,
    source_type: SourceType::Wikidata,
    name: "TheDraw Pascal screen Image",
    extensions: &["imm", "pas"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7B, 0x20, 0x54, 0x68, 0x65, 0x44, 0x72, 0x61, 0x77, 0x20, 0x50, 0x61, 0x73,
                    0x63, 0x61, 0x6C, 0x20, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6E, 0x20, 0x49, 0x6D,
                    0x61, 0x67, 0x65, 0x20, 0x7D, 0x0D, 0x0A, 0x63, 0x6F, 0x6E, 0x73, 0x74, 0x0D,
                    0x0A, 0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

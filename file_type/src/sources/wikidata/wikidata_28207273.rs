use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207273: FileFormat = FileFormat {
    id: 28_207_273,
    source_type: SourceType::Wikidata,
    name: "SGX Graphics File Format",
    extensions: &["sgx", "svg"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x47, 0x58, 0x20, 0x47, 0x72, 0x61, 0x70, 0x68, 0x69, 0x63, 0x73, 0x20,
                    0x46, 0x69, 0x6C, 0x65, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857620: FileFormat = FileFormat {
    id: 105_857_620,
    source_type: SourceType::Wikidata,
    name: "Deluxe Sound sampled instrument",
    extensions: &["instr"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x61, 0x6D, 0x70, 0x6C, 0x65, 0x64, 0x53, 0x6F, 0x75, 0x6E, 0x64,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

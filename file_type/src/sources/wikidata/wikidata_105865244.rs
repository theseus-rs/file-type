use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865244: FileFormat = FileFormat {
    id: 105_865_244,
    source_type: SourceType::Wikidata,
    name: "PICO-8 cartridge",
    extensions: &["p8"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x70, 0x69, 0x63, 0x6F, 0x2D, 0x38, 0x20, 0x63, 0x61, 0x72, 0x74, 0x72, 0x69,
                    0x64, 0x67, 0x65, 0x20, 0x2F, 0x2F, 0x20, 0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F,
                    0x2F, 0x77, 0x77, 0x77, 0x2E, 0x70, 0x69, 0x63, 0x6F, 0x2D, 0x38, 0x2E, 0x63,
                    0x6F, 0x6D, 0x0A, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

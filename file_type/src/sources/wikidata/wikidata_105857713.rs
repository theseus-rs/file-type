use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857713: FileFormat = FileFormat {
    id: 105_857_713,
    source_type: SourceType::Wikidata,
    name: "Microsoft Zone Identifier",
    extensions: &["identifier"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x5A, 0x6F, 0x6E, 0x65, 0x54, 0x72, 0x61, 0x6E, 0x73, 0x66, 0x65, 0x72,
                    0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

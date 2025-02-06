use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857717: FileFormat = FileFormat {
    id: 105_857_717,
    source_type: SourceType::Wikidata,
    name: "Microsoft Developer intermediate file format",
    extensions: &["idb"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x20, 0x43, 0x2F, 0x43,
                    0x2B, 0x2B, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

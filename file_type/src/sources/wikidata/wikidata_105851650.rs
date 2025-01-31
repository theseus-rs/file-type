use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851650: FileFormat = FileFormat {
    id: 105_851_650,
    puid: "wikidata/105851650",
    name: "Microsoft SZ compressed",
    extensions: &["ex$"],
    media_types: &["application/x-ms-compress-sz"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x5A, 0x20, 0x88, 0xF0, 0x27, 0x33, 0xD1,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

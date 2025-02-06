use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855591: FileFormat = FileFormat {
    id: 105_855_591,
    source_type: SourceType::Wikidata,
    name: "Oxygene Project (UTF-8)",
    extensions: &["oxygene"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x3C, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x20, 0x44,
                    0x65, 0x66, 0x61, 0x75, 0x6C, 0x74, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74, 0x73,
                    0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

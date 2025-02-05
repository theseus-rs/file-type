use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864540: FileFormat = FileFormat {
    id: 105_864_540,
    source_type: SourceType::Wikidata,
    name: "DCS device Profile",
    extensions: &["pr0"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFE, 0x5B, 0x00, 0x70, 0x00, 0x72, 0x00, 0x6F, 0x00, 0x66, 0x00, 0x69,
                    0x00, 0x6C, 0x00, 0x65, 0x00, 0x3D, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

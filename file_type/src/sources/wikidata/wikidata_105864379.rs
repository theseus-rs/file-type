use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864379: FileFormat = FileFormat {
    id: 105_864_379,
    source_type: SourceType::Wikidata,
    name: "Gambas Project",
    extensions: &["project"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x47, 0x61, 0x6D, 0x62, 0x61, 0x73, 0x20, 0x50, 0x72, 0x6F, 0x6A,
                    0x65, 0x63, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

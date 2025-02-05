use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854337: FileFormat = FileFormat {
    id: 105_854_337,
    source_type: SourceType::Wikidata,
    name: "Xpack compressed archive",
    extensions: &["xpa"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x78, 0x70, 0x61, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x6A, 0x6D, 0x74, 0x18,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

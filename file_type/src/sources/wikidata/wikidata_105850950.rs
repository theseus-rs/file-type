use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850950: FileFormat = FileFormat {
    id: 105_850_950,
    source_type: SourceType::Wikidata,
    name: "Win994a tape image",
    extensions: &["titape"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x49, 0x2D, 0x54, 0x41, 0x50, 0x45, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

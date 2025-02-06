use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850719: FileFormat = FileFormat {
    id: 105_850_719,
    source_type: SourceType::Wikidata,
    name: "KetabeAvval Data Format",
    extensions: &["kadf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4B, 0x65, 0x74, 0x61, 0x62, 0x65, 0x41, 0x76, 0x76, 0x61, 0x6C, 0x20, 0x44,
                    0x61, 0x74, 0x61, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859291: FileFormat = FileFormat {
    id: 105_859_291,
    source_type: SourceType::Wikidata,
    name: "FLIR Public Format bitmap",
    extensions: &["fpf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x50, 0x46, 0x20, 0x50, 0x75, 0x62, 0x6C, 0x69, 0x63, 0x20, 0x49, 0x6D,
                    0x61, 0x67, 0x65, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

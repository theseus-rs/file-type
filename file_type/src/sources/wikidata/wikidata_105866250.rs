use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866250: FileFormat = FileFormat {
    id: 105_866_250,
    source_type: SourceType::Wikidata,
    name: "Sprite Backup image",
    extensions: &["pbf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x4F, 0x43, 0x4B, 0x45, 0x54, 0x50, 0x43, 0x49, 0x4D, 0x41, 0x47, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

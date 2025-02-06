use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855405: FileFormat = FileFormat {
    id: 105_855_405,
    source_type: SourceType::Wikidata,
    name: "Fallout 3 save game",
    extensions: &["fos"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x4F, 0x33, 0x53, 0x41, 0x56, 0x45, 0x47, 0x41, 0x4D, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

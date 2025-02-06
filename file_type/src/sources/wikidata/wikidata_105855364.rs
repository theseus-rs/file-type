use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855364: FileFormat = FileFormat {
    id: 105_855_364,
    source_type: SourceType::Wikidata,
    name: "Kodak Camera Firmware",
    extensions: &["fw"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x32, 0x37, 0x4D, 0x41, 0x49, 0x4E, 0x20, 0x46, 0x49, 0x52, 0x4D, 0x57, 0x41,
                    0x52, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

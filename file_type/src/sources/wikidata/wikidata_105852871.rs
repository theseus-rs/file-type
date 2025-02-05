use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852871: FileFormat = FileFormat {
    id: 105_852_871,
    source_type: SourceType::Wikidata,
    name: "Garmin vechicle images file",
    extensions: &["srf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x41, 0x52, 0x4D, 0x49, 0x4E, 0x20, 0x42, 0x49, 0x54, 0x4D, 0x41, 0x50,
                    0x20, 0x30, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

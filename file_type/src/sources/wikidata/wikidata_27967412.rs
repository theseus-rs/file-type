use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967412: FileFormat = FileFormat {
    id: 27_967_412,
    puid: "wikidata/27967412",
    name: "Sound Blaster Instrument",
    extensions: &["sbi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x42, 0x49, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};

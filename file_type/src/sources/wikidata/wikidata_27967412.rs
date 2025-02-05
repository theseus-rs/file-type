use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967412: FileFormat = FileFormat {
    id: 27_967_412,
    source_type: SourceType::Wikidata,
    name: "Sound Blaster Instrument",
    extensions: &["sbi"],
    media_types: &[],
    signatures: &[Signature {
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

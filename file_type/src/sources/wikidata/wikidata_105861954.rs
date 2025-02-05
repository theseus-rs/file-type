use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861954: FileFormat = FileFormat {
    id: 105_861_954,
    source_type: SourceType::Wikidata,
    name: "MEMU Floppy image",
    extensions: &["mfloppy"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xC3, 0x40, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};

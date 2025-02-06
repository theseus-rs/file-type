use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865646: FileFormat = FileFormat {
    id: 105_865_646,
    source_type: SourceType::Wikidata,
    name: "Brother/Babylock/Bernina Home Embroidery format",
    extensions: &["pec"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x50, 0x45, 0x43, 0x30, 0x30, 0x30, 0x31, 0x4C, 0x41, 0x3A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

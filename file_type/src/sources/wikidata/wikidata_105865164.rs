use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865164: FileFormat = FileFormat {
    id: 105_865_164,
    source_type: SourceType::Wikidata,
    name: "Thinkbox Particle",
    extensions: &["prt"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xC0, 0x50, 0x52, 0x54, 0x0D, 0x0A, 0x1A, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

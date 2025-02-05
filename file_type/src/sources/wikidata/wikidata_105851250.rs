use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851250: FileFormat = FileFormat {
    id: 105_851_250,
    source_type: SourceType::Wikidata,
    name: "Acronis True Image",
    extensions: &["tib"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB4, 0x6E, 0x68, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};

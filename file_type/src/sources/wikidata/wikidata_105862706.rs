use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862706: FileFormat = FileFormat {
    id: 105_862_706,
    source_type: SourceType::Wikidata,
    name: "Fantavision Movie (v1.00 MS-DOS)",
    extensions: &["mve"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xB4, 0x05])],
            },
        }],
    }],
    related_formats: &[],
};

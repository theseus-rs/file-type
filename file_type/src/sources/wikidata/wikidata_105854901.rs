use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854901: FileFormat = FileFormat {
    id: 105_854_901,
    source_type: SourceType::Wikidata,
    name: "Unpackaged Authorware 3 for Windows file",
    extensions: &["a3w"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x43, 0x52, 0x53, 0xA8, 0xBC, 0xAD, 0xAC,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

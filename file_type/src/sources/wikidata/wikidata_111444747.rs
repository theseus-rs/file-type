use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111444747: FileFormat = FileFormat {
    id: 111_444_747,
    source_type: SourceType::Wikidata,
    name: "Leica Geosystems universal digital reality file",
    extensions: &["lgs"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x77, 0x69, 0x6C, 0x64, 0x3A, 0x3A, 0x67, 0x65, 0x6F, 0x73, 0x79, 0x73, 0x74,
                    0x65, 0x6D, 0x73,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

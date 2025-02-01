use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856950: FileFormat = FileFormat {
    id: 105_856_950,
    puid: "wikidata/105856950",
    name: "Micro Focus COBOL generated code",
    extensions: &["gnt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x69, 0x63, 0x72, 0x6F, 0x20, 0x46, 0x6F, 0x63, 0x75, 0x73, 0x20, 0x43,
                    0x4F, 0x42, 0x4F, 0x4C, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

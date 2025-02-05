use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28445577: FileFormat = FileFormat {
    id: 28_445_577,
    source_type: SourceType::Wikidata,
    name: "Advanced Authoring System",
    extensions: &["aas"],
    media_types: &["application/x-authorware-seg"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861629: FileFormat = FileFormat {
    id: 105_861_629,
    puid: "wikidata/105861629",
    name: "Micro Focus COBOL Library",
    extensions: &["lbr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x69, 0x63, 0x72, 0x6F, 0x20, 0x46, 0x6F, 0x63, 0x75, 0x73, 0x20, 0x43,
                    0x4F, 0x42, 0x4F, 0x4C, 0x20, 0x4C, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x20,
                    0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

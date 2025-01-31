use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859713: FileFormat = FileFormat {
    id: 105_859_713,
    puid: "wikidata/105859713",
    name: "X86 Delta Compiler Video",
    extensions: &["xdv"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x44, 0x43, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};

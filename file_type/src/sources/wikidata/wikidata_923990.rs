use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_923990: FileFormat = FileFormat {
    id: 923_990,
    puid: "wikidata/923990",
    name: "RIS",
    extensions: &["ris"],
    media_types: &["application/x-research-info-systems"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::WildcardCountRange(0, 128),
                    Token::Literal(&[0x54, 0x59, 0x20]),
                ],
            },
        }],
    }],
    related_formats: &[],
};

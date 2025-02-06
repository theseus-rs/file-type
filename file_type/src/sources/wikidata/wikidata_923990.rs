use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_923990: FileFormat = FileFormat {
    id: 923_990,
    source_type: SourceType::Wikidata,
    name: "RIS",
    extensions: &["ris"],
    media_types: &["application/x-research-info-systems"],
    signatures: &[Signature {
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

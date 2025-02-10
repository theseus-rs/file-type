use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_382011: FileFormat = FileFormat {
    id: 382_011,
    source_type: SourceType::Wikidata,
    name: "Program information file",
    extensions: &["pif"],
    media_types: &["application/x-pif"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00])],
            },
        }],
    }],
    related_formats: &[],
};

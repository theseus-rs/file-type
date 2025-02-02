use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865074: FileFormat = FileFormat {
    id: 105_865_074,
    source_type: SourceType::Wikidata,
    name: "CODESYS Project",
    extensions: &["pro"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x6F, 0x44, 0x65, 0x53, 0x79, 0x73, 0x2B, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856502: FileFormat = FileFormat {
    id: 105_856_502,
    source_type: SourceType::Wikidata,
    name: "Wii Disc File (v1)",
    extensions: &["wdf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x49, 0x49, 0x01, 0x44, 0x49, 0x53, 0x43, 0x00, 0x00, 0x00, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851868: FileFormat = FileFormat {
    id: 105_851_868,
    source_type: SourceType::Wikidata,
    name: "StoneCracker S403 compressed",
    extensions: &["stc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x34, 0x30, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};

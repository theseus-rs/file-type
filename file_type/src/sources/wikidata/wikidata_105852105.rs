use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852105: FileFormat = FileFormat {
    id: 105_852_105,
    source_type: SourceType::Wikidata,
    name: "StoneCracker S401 compressed",
    extensions: &["stc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x34, 0x30, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};

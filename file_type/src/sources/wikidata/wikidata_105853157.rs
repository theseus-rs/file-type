use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853157: FileFormat = FileFormat {
    id: 105_853_157,
    source_type: SourceType::Wikidata,
    name: "StoneCracker S404 compressed",
    extensions: &["stc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x34, 0x30, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};

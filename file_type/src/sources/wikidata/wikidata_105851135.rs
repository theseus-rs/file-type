use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851135: FileFormat = FileFormat {
    id: 105_851_135,
    source_type: SourceType::Wikidata,
    name: "TRSI Sound Monitor song",
    extensions: &["tsm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x53, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};

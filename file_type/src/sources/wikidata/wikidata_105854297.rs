use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854297: FileFormat = FileFormat {
    id: 105_854_297,
    source_type: SourceType::Wikidata,
    name: "FMOD Sample Bank format (v4)",
    extensions: &["fsb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x53, 0x42, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};

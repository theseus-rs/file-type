use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860206: FileFormat = FileFormat {
    id: 105_860_206,
    source_type: SourceType::Wikidata,
    name: "BlackBarry encrypted REM format",
    extensions: &["rem"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x45, 0x4D, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};

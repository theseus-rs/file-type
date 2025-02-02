use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854793: FileFormat = FileFormat {
    id: 105_854_793,
    source_type: SourceType::Wikidata,
    name: "Compressia Archive",
    extensions: &["car"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x4D, 0x50, 0x30, 0x43, 0x4D, 0x50, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

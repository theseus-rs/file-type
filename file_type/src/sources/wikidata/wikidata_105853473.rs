use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853473: FileFormat = FileFormat {
    id: 105_853_473,
    source_type: SourceType::Wikidata,
    name: "EightyOne snapshot",
    extensions: &["z81"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x43, 0x50, 0x55, 0x5D, 0x0D, 0x0A, 0x50, 0x43, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

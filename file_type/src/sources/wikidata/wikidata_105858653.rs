use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858653: FileFormat = FileFormat {
    id: 105_858_653,
    source_type: SourceType::Wikidata,
    name: "Bagpipe notation",
    extensions: &["bww"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x61, 0x67, 0x70, 0x69, 0x70, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856847: FileFormat = FileFormat {
    id: 105_856_847,
    source_type: SourceType::Wikidata,
    name: "GFA-BASIC Atari v3.5-4.x tokenized source",
    extensions: &["gfa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x04, 0x47, 0x46, 0x41, 0x2D, 0x42, 0x41, 0x53, 0x49, 0x43, 0x33,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

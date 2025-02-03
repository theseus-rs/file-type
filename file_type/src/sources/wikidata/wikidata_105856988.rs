use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856988: FileFormat = FileFormat {
    id: 105_856_988,
    source_type: SourceType::Wikidata,
    name: "GFA-BASIC Amiga tokenized source",
    extensions: &["gfa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x03, 0x47, 0x46, 0x41, 0x2D, 0x41, 0x4D, 0x49, 0x47, 0x41, 0x42,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

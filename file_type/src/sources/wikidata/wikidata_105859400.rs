use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859400: FileFormat = FileFormat {
    id: 105_859_400,
    source_type: SourceType::Wikidata,
    name: "Statler Stitcher",
    extensions: &["qli"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x43, 0x41, 0x44, 0x32, 0x44, 0x4D, 0x43, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

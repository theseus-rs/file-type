use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856908: FileFormat = FileFormat {
    id: 105_856_908,
    source_type: SourceType::Wikidata,
    name: "GFA-BASIC MS-DOS tokenized source",
    extensions: &["gfa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x46, 0x41, 0x2D, 0x42, 0x41, 0x53, 0x49, 0x43, 0x20, 0x4D, 0x53, 0x2D,
                    0x44, 0x4F, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

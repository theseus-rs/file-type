use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109370242: FileFormat = FileFormat {
    id: 109_370_242,
    source_type: SourceType::Wikidata,
    name: "XRes Multi-resolution Bitmap",
    extensions: &["lrg"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x58, 0x52, 0x45, 0x53, 0x20, 0x4C, 0x41, 0x52, 0x47, 0x45, 0x20, 0x46, 0x4F,
                    0x52, 0x4D, 0x41, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

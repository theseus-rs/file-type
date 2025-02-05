use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855077: FileFormat = FileFormat {
    id: 105_855_077,
    source_type: SourceType::Wikidata,
    name: "CA Visual Object Application Export File",
    extensions: &["aef"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x20, 0x00, 0x00, 0x00, 0x80, 0x02, 0x43, 0x41, 0x2D, 0x56, 0x4F,
                    0x20, 0x41, 0x70, 0x70, 0x6C, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20,
                    0x45, 0x78, 0x70, 0x6F, 0x72, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857562: FileFormat = FileFormat {
    id: 105_857_562,
    source_type: SourceType::Wikidata,
    name: "2D spline geometry",
    extensions: &["in2d"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x73, 0x70, 0x6C, 0x69, 0x6E, 0x65, 0x63, 0x75, 0x72, 0x76, 0x65, 0x73, 0x32,
                    0x64, 0x76, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

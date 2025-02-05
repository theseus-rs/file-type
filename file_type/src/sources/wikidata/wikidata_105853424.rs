use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853424: FileFormat = FileFormat {
    id: 105_853_424,
    source_type: SourceType::Wikidata,
    name: "SketchChair document",
    extensions: &["skchr"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x53, 0x6B, 0x65, 0x74, 0x63, 0x68, 0x43, 0x68, 0x61, 0x69, 0x72, 0x44,
                    0x6F, 0x63, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

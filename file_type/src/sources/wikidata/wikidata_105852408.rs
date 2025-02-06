use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852408: FileFormat = FileFormat {
    id: 105_852_408,
    source_type: SourceType::Wikidata,
    name: "ProfiCAD drawing (v4)",
    extensions: &["sxe"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x50, 0x72, 0x6F, 0x66, 0x69, 0x43, 0x41, 0x44, 0x20, 0x53,
                    0x58, 0x45, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x34, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

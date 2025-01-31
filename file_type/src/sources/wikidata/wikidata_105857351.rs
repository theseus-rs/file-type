use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857351: FileFormat = FileFormat {
    id: 105_857_351,
    puid: "wikidata/105857351",
    name: "PALASM (var.2)",
    extensions: &["jed"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0D, 0x0A, 0x50, 0x41, 0x4C, 0x41, 0x53, 0x4D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

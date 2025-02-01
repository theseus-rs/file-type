use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857402: FileFormat = FileFormat {
    id: 105_857_402,
    puid: "wikidata/105857402",
    name: "Japan Crossword File (Dmitry Torshin format)",
    extensions: &["jcc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4A, 0x41, 0x50, 0x41, 0x4E, 0x02, 0xFF, 0xFF, 0xFF,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

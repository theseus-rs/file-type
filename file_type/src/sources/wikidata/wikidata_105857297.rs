use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857297: FileFormat = FileFormat {
    id: 105_857_297,
    source_type: SourceType::Wikidata,
    name: "CZ Help",
    extensions: &["hlp"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x5A, 0x5F, 0x48, 0x45, 0x4C, 0x50, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

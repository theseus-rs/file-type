use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859189: FileFormat = FileFormat {
    id: 105_859_189,
    source_type: SourceType::Wikidata,
    name: "HP ASII GROB bitmap (embedded)",
    extensions: &["asc", "grb", "gro"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x25, 0x48, 0x50, 0x3A, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};

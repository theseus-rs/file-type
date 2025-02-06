use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854029: FileFormat = FileFormat {
    id: 105_854_029,
    source_type: SourceType::Wikidata,
    name: "lzop compressed",
    extensions: &["lzo"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x89, 0x4C, 0x5A, 0x4F, 0x00, 0x0D, 0x0A, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

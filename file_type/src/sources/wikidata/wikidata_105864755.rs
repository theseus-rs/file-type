use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864755: FileFormat = FileFormat {
    id: 105_864_755,
    source_type: SourceType::Wikidata,
    name: "DIV Games Studio Palette",
    extensions: &["pal"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x70, 0x61, 0x6C, 0x1A, 0x0D, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};

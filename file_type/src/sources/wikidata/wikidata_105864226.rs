use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864226: FileFormat = FileFormat {
    id: 105_864_226,
    source_type: SourceType::Wikidata,
    name: "PS2DIS project",
    extensions: &["pis"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x49, 0x53, 0x1F])],
            },
        }],
    }],
    related_formats: &[],
};

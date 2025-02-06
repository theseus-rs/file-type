use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864209: FileFormat = FileFormat {
    id: 105_864_209,
    source_type: SourceType::Wikidata,
    name: "Panasonic SD Voice Editor file",
    extensions: &["plm"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x31, 0x00, 0x00, 0x53, 0x44, 0x2D, 0x56, 0x4F, 0x49, 0x43, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

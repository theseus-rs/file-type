use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858083: FileFormat = FileFormat {
    id: 105_858_083,
    source_type: SourceType::Wikidata,
    name: "WOZ disk image (v1)",
    extensions: &["woz"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x4F, 0x5A, 0x31, 0xFF, 0x0A, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

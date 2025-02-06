use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856316: FileFormat = FileFormat {
    id: 105_856_316,
    source_type: SourceType::Wikidata,
    name: "Macintosh encrypted Disk image (v1)",
    extensions: &["dmg"],
    media_types: &["application/x-apple-diskimage"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x64, 0x73, 0x61, 0x65, 0x6E, 0x63, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849623: FileFormat = FileFormat {
    id: 105_849_623,
    source_type: SourceType::Wikidata,
    name: "SFX Advanced Calculator spreadsheet",
    extensions: &["calc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x58, 0x46, 0x2D, 0x43, 0x63, 0x30, 0x30, 0x4F, 0x70, 0x74, 0x73,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

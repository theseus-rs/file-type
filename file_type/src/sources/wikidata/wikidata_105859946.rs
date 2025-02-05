use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859946: FileFormat = FileFormat {
    id: 105_859_946,
    source_type: SourceType::Wikidata,
    name: "Psygnosis Video",
    extensions: &["vid"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x4F, 0x52, 0x4D, 0x50, 0x53, 0x59, 0x47, 0x78, 0x78, 0x78, 0x78, 0x53,
                    0x41, 0x4E, 0x4D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

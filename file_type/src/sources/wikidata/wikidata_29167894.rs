use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167894: FileFormat = FileFormat {
    id: 29_167_894,
    source_type: SourceType::Wikidata,
    name: "Personal Ancestral File, version 5",
    extensions: &["paf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x35, 0x30, 0x30, 0x00, 0x35, 0x30, 0x30, 0x00, 0x50, 0x41, 0x46,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

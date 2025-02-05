use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857347: FileFormat = FileFormat {
    id: 105_857_347,
    source_type: SourceType::Wikidata,
    name: "Pro Video Job",
    extensions: &["job"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x56, 0x43, 0x47, 0x48, 0x45, 0x41, 0x44, 0x46, 0x4F, 0x4E, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

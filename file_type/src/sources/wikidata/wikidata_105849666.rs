use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849666: FileFormat = FileFormat {
    id: 105_849_666,
    source_type: SourceType::Wikidata,
    name: "CPBackup Configuration (v9.x)",
    extensions: &["cfg"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x43, 0x42, 0x53, 0x49, 0x47, 0x5D, 0x39,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

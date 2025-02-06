use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852804: FileFormat = FileFormat {
    id: 105_852_804,
    source_type: SourceType::Wikidata,
    name: "CPBackup backup Settings (v7.x)",
    extensions: &["set"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x50, 0x43, 0x42, 0x53, 0x49, 0x47, 0x5D, 0x37,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

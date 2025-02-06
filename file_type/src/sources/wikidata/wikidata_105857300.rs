use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857300: FileFormat = FileFormat {
    id: 105_857_300,
    source_type: SourceType::Wikidata,
    name: "HTC SMS Backup",
    extensions: &["hbk"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x54, 0x43, 0x4D, 0x53, 0x47, 0x42, 0x41, 0x43, 0x4B, 0x55, 0x50, 0x5F,
                    0x56, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

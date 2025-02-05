use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857637: FileFormat = FileFormat {
    id: 105_857_637,
    source_type: SourceType::Wikidata,
    name: "PC-Fullbak volume disk image",
    extensions: &["dsk"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x56, 0x6F, 0x6C, 0x48, 0x64, 0x72, 0x73, 0x63,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

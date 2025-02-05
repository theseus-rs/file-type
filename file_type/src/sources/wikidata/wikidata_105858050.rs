use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858050: FileFormat = FileFormat {
    id: 105_858_050,
    source_type: SourceType::Wikidata,
    name: "ERC Virtual Floppy Disk image",
    extensions: &["vfd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x52, 0x43, 0x56, 0x46, 0x44, 0x30, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

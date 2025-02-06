use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849615: FileFormat = FileFormat {
    id: 105_849_615,
    source_type: SourceType::Wikidata,
    name: "CompuColor Virtual Floppy disk image",
    extensions: &["ccvf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x6F, 0x6D, 0x70, 0x75, 0x63, 0x6F, 0x6C, 0x6F, 0x72, 0x20, 0x56, 0x69,
                    0x72, 0x74, 0x75, 0x61, 0x6C, 0x20, 0x46, 0x6C, 0x6F, 0x70, 0x70, 0x79, 0x20,
                    0x44, 0x69, 0x73, 0x6B, 0x20, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x0D, 0x0A, 0x4C,
                    0x61, 0x62, 0x65, 0x6C, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

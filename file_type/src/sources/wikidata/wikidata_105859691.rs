use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859691: FileFormat = FileFormat {
    id: 105_859_691,
    source_type: SourceType::Wikidata,
    name: "VirtualBox Disk Image (Innotek)",
    extensions: &["vdi"],
    media_types: &["application/x-virtualbox-vdi"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x3C, 0x3C, 0x20, 0x69, 0x6E, 0x6E, 0x6F, 0x74, 0x65, 0x6B, 0x20, 0x56,
                    0x69, 0x72, 0x74, 0x75, 0x61, 0x6C, 0x42, 0x6F, 0x78, 0x20, 0x44, 0x69, 0x73,
                    0x6B, 0x20, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x20, 0x3E, 0x3E, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

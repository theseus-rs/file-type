use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852067: FileFormat = FileFormat {
    id: 105_852_067,
    source_type: SourceType::Wikidata,
    name: "SimplePlanes compressed Mod",
    extensions: &["spmod"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x69, 0x6D, 0x70, 0x6C, 0x65, 0x50, 0x6C, 0x61, 0x6E, 0x65, 0x73, 0x43,
                    0x6F, 0x6D, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x4D, 0x6F, 0x64, 0x46,
                    0x69, 0x6C, 0x65, 0x56,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857608: FileFormat = FileFormat {
    id: 105_857_608,
    source_type: SourceType::Wikidata,
    name: "TraxPlayer audio floppy image",
    extensions: &["dsk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEB, 0xFE, 0x90, 0x54, 0x72, 0x61, 0x78, 0x50, 0x6C, 0x61, 0x79,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

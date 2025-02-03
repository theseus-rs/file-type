use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857289: FileFormat = FileFormat {
    id: 105_857_289,
    source_type: SourceType::Wikidata,
    name: "HotHelp Header",
    extensions: &["hdr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x24, 0x56, 0x45, 0x52, 0x3A, 0x20, 0x48, 0x6F, 0x74, 0x48, 0x65, 0x6C, 0x70,
                    0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

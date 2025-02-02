use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852237: FileFormat = FileFormat {
    id: 105_852_237,
    source_type: SourceType::Wikidata,
    name: "Sasami Script subtitles",
    extensions: &["s2k"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3B, 0x45, 0x6E, 0x76, 0x2E, 0x4D, 0x6F, 0x76, 0x69, 0x65, 0x2E, 0x57, 0x69,
                    0x64, 0x74, 0x68, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

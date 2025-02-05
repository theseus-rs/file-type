use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863800: FileFormat = FileFormat {
    id: 105_863_800,
    source_type: SourceType::Wikidata,
    name: "Management Pack Fragment (UTF-8)",
    extensions: &["mpx"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xEF, 0xBB, 0xBF, 0x3C, 0x4D, 0x61, 0x6E, 0x61, 0x67, 0x65, 0x6D, 0x65, 0x6E,
                    0x74, 0x50, 0x61, 0x63, 0x6B, 0x46, 0x72, 0x61, 0x67, 0x6D, 0x65, 0x6E, 0x74,
                    0x20, 0x53, 0x63, 0x68, 0x65, 0x6D, 0x61, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F,
                    0x6E, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

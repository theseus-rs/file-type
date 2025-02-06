use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863525: FileFormat = FileFormat {
    id: 105_863_525,
    source_type: SourceType::Wikidata,
    name: "Mighty Draw Windows library",
    extensions: &["mwl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x69, 0x67, 0x68, 0x74, 0x79, 0x20, 0x44, 0x72, 0x61, 0x77, 0x20, 0x57,
                    0x69, 0x6E, 0x64, 0x6F, 0x77, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

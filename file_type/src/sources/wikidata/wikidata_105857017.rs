use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857017: FileFormat = FileFormat {
    id: 105_857_017,
    puid: "wikidata/105857017",
    name: "GFA-BASIC Windows v3 tokenized source",
    extensions: &["gfa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x46, 0x41, 0x2D, 0x42, 0x41, 0x53, 0x49, 0x43, 0x20, 0x57, 0x49, 0x4E,
                    0x33, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

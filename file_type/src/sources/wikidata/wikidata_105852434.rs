use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852434: FileFormat = FileFormat {
    id: 105_852_434,
    puid: "wikidata/105852434",
    name: "SubRip subtitles (UTF-16)",
    extensions: &["srt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0xFE, 0x31, 0x00, 0x0D])],
            },
        }],
    }],
    related_formats: &[],
};

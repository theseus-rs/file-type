use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855269: FileFormat = FileFormat {
    id: 105_855_269,
    source_type: SourceType::Wikidata,
    name: "Fritzing shareable project",
    extensions: &["fzz"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4B, 0x03, 0x04])],
            },
        }],
    }],
    related_formats: &[],
};

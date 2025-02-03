use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854086: FileFormat = FileFormat {
    id: 105_854_086,
    source_type: SourceType::Wikidata,
    name: "Jet-VoiceMail audio data",
    extensions: &["cvf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4F, 0x57, 0x4F, 0x4E, 0x56, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};

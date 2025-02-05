use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862489: FileFormat = FileFormat {
    id: 105_862_489,
    source_type: SourceType::Wikidata,
    name: "Fred Editor SoundTrack",
    extensions: &["audio/x-mod"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x72, 0x65, 0x64, 0x20, 0x45, 0x64, 0x69, 0x74, 0x6F, 0x72, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

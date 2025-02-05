use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111440891: FileFormat = FileFormat {
    id: 111_440_891,
    source_type: SourceType::Wikidata,
    name: "VoiceXML File",
    extensions: &["vxml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

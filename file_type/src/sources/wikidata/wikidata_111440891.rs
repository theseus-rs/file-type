use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111440891: FileFormat = FileFormat {
    id: 111_440_891,
    source_type: SourceType::Wikidata,
    name: "VoiceXML File",
    extensions: &["vxml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

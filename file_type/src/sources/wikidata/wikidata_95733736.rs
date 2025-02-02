use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_95733736: FileFormat = FileFormat {
    id: 95_733_736,
    source_type: SourceType::Wikidata,
    name: "RealAudio Metafile",
    extensions: &["ram"],
    media_types: &["audio/vnd.rn-realaudio", "audio/x-pn-realaudio"],
    internal_signatures: &[],
    related_formats: &[],
};

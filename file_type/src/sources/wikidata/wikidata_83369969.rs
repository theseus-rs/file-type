use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_83369969: FileFormat = FileFormat {
    id: 83_369_969,
    source_type: SourceType::Wikidata,
    name: "FastTracker2 Extended",
    extensions: &["xi", "xm"],
    media_types: &["audio/x-xi", "audio/x-xm"],
    internal_signatures: &[],
    related_formats: &[],
};

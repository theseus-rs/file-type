use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83369969: FileFormat = FileFormat {
    id: 83_369_969,
    source_type: SourceType::Wikidata,
    name: "FastTracker2 Extended",
    extensions: &["xi", "xm"],
    media_types: &["audio/x-xi", "audio/x-xm"],
    signatures: &[],
    related_formats: &[],
};

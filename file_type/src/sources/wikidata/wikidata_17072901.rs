use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_17072901: FileFormat = FileFormat {
    id: 17_072_901,
    source_type: SourceType::Wikidata,
    name: "Open Game Engine Exchange",
    extensions: &["ogex"],
    media_types: &["model/vnd.opengex"],
    signatures: &[],
    related_formats: &[],
};

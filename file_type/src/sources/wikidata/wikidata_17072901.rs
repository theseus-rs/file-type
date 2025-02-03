use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_17072901: FileFormat = FileFormat {
    id: 17_072_901,
    source_type: SourceType::Wikidata,
    name: "Open Game Engine Exchange",
    extensions: &["ogex"],
    media_types: &["model/vnd.opengex"],
    internal_signatures: &[],
    related_formats: &[],
};

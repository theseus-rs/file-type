use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117804274: FileFormat = FileFormat {
    id: 117_804_274,
    source_type: SourceType::Wikidata,
    name: "VideoImpression mini-player",
    extensions: &["exe"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

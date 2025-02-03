use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975912: FileFormat = FileFormat {
    id: 28_975_912,
    source_type: SourceType::Wikidata,
    name: "XGL",
    extensions: &["xgl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

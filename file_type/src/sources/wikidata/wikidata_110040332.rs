use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110040332: FileFormat = FileFormat {
    id: 110_040_332,
    source_type: SourceType::Wikidata,
    name: "Harvard Graphics Presentation, version 1-3 PRS",
    extensions: &["prs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

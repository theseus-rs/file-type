use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118139198: FileFormat = FileFormat {
    id: 118_139_198,
    source_type: SourceType::Wikidata,
    name: "DOS Caledar File",
    extensions: &["cal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

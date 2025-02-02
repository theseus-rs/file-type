use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205925: FileFormat = FileFormat {
    id: 28_205_925,
    source_type: SourceType::Wikidata,
    name: "Doodle",
    extensions: &["doo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66663925: FileFormat = FileFormat {
    id: 66_663_925,
    source_type: SourceType::Wikidata,
    name: "OS/2 Metafile",
    extensions: &["met"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

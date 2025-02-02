use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127805343: FileFormat = FileFormat {
    id: 127_805_343,
    source_type: SourceType::Wikidata,
    name: "njs script file",
    extensions: &["njs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127518715: FileFormat = FileFormat {
    id: 127_518_715,
    source_type: SourceType::Wikidata,
    name: "Zephir source code file",
    extensions: &["zep"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

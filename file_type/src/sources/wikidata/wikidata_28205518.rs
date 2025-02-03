use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205518: FileFormat = FileFormat {
    id: 28_205_518,
    source_type: SourceType::Wikidata,
    name: "atomix.scores",
    extensions: &["scores"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

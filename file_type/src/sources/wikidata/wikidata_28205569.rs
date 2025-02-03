use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205569: FileFormat = FileFormat {
    id: 28_205_569,
    source_type: SourceType::Wikidata,
    name: "Nokia Startup Logo",
    extensions: &["nsl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

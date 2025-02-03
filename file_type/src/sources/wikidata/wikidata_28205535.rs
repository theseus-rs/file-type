use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205535: FileFormat = FileFormat {
    id: 28_205_535,
    source_type: SourceType::Wikidata,
    name: "iThmb",
    extensions: &["ithmb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

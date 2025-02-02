use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111166086: FileFormat = FileFormat {
    id: 111_166_086,
    source_type: SourceType::Wikidata,
    name: "Songbase File",
    extensions: &["sngbase"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

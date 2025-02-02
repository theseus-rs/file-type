use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967179: FileFormat = FileFormat {
    id: 27_967_179,
    source_type: SourceType::Wikidata,
    name: "Farandole Form 2.0",
    extensions: &["f2r"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

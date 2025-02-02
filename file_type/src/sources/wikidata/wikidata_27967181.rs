use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967181: FileFormat = FileFormat {
    id: 27_967_181,
    source_type: SourceType::Wikidata,
    name: "Farandole Composer pattern",
    extensions: &["fpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

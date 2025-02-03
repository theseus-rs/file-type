use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_74674755: FileFormat = FileFormat {
    id: 74_674_755,
    source_type: SourceType::Wikidata,
    name: "SPSS Table Look",
    extensions: &["tlo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

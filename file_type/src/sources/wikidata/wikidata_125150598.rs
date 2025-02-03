use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125150598: FileFormat = FileFormat {
    id: 125_150_598,
    source_type: SourceType::Wikidata,
    name: "Gliffy diagram (gxml)",
    extensions: &["gxml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

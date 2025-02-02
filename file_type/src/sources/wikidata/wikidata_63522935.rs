use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_63522935: FileFormat = FileFormat {
    id: 63_522_935,
    source_type: SourceType::Wikidata,
    name: "Parametric Technology Pro/ENGINEER File Format",
    extensions: &["prt"],
    media_types: &["application/pro_eng"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130062600: FileFormat = FileFormat {
    id: 130_062_600,
    source_type: SourceType::Wikidata,
    name: "Kal source code file",
    extensions: &["kal"],
    media_types: &["application/kal", "text/kal"],
    internal_signatures: &[],
    related_formats: &[],
};

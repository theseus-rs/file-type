use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_5227180: FileFormat = FileFormat {
    id: 5_227_180,
    source_type: SourceType::Wikidata,
    name: "Data Interchange Format",
    extensions: &["dif"],
    media_types: &["application/x-dif"],
    internal_signatures: &[],
    related_formats: &[],
};

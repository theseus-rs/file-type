use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5227180: FileFormat = FileFormat {
    id: 5_227_180,
    source_type: SourceType::Wikidata,
    name: "Data Interchange Format",
    extensions: &["dif"],
    media_types: &["application/x-dif"],
    signatures: &[],
    related_formats: &[],
};

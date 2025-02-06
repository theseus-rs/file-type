use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128693712: FileFormat = FileFormat {
    id: 128_693_712,
    source_type: SourceType::Wikidata,
    name: "Berry source code file",
    extensions: &["be"],
    media_types: &["application/x-berry", "text/x-berry"],
    signatures: &[],
    related_formats: &[],
};

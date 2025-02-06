use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130736862: FileFormat = FileFormat {
    id: 130_736_862,
    source_type: SourceType::Wikidata,
    name: "Scalate Server Page file",
    extensions: &["ssp"],
    media_types: &["application/x-ssp"],
    signatures: &[],
    related_formats: &[],
};

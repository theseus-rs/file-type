use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27350220: FileFormat = FileFormat {
    id: 27_350_220,
    source_type: SourceType::Wikidata,
    name: "ADRG General Information File",
    extensions: &["gen"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

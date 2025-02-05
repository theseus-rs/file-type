use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118140134: FileFormat = FileFormat {
    id: 118_140_134,
    source_type: SourceType::Wikidata,
    name: "Serenade Project File",
    extensions: &["ssp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967105: FileFormat = FileFormat {
    id: 27_967_105,
    source_type: SourceType::Wikidata,
    name: "SHO",
    extensions: &["sho"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

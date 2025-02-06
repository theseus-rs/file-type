use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_40410022: FileFormat = FileFormat {
    id: 40_410_022,
    source_type: SourceType::Wikidata,
    name: "Feather",
    extensions: &["feather"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

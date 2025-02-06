use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52063276: FileFormat = FileFormat {
    id: 52_063_276,
    source_type: SourceType::Wikidata,
    name: "SAP Document",
    extensions: &["ali"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

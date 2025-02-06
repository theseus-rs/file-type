use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62414875: FileFormat = FileFormat {
    id: 62_414_875,
    source_type: SourceType::Wikidata,
    name: "XAML Binary Format",
    extensions: &["xbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

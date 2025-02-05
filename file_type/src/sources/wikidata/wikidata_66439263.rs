use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66439263: FileFormat = FileFormat {
    id: 66_439_263,
    source_type: SourceType::Wikidata,
    name: "Word Perfect Templates file format",
    extensions: &["wpt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

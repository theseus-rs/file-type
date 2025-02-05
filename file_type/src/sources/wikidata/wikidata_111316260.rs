use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111316260: FileFormat = FileFormat {
    id: 111_316_260,
    source_type: SourceType::Wikidata,
    name: "Sample Cell II Mac instrument",
    extensions: &["ini"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

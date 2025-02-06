use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48804265: FileFormat = FileFormat {
    id: 48_804_265,
    source_type: SourceType::Wikidata,
    name: "Paradox Database Memo Field",
    extensions: &["dbq", "mb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

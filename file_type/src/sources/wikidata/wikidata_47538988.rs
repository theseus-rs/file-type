use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47538988: FileFormat = FileFormat {
    id: 47_538_988,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Last Saved Layer State",
    extensions: &["las"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

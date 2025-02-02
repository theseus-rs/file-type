use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47538988: FileFormat = FileFormat {
    id: 47_538_988,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Last Saved Layer State",
    extensions: &["las"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

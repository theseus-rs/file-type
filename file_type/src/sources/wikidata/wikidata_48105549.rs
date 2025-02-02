use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48105549: FileFormat = FileFormat {
    id: 48_105_549,
    source_type: SourceType::Wikidata,
    name: "SAS for MS-DOS Catalog",
    extensions: &["sct"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

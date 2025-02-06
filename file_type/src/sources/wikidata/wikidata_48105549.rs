use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48105549: FileFormat = FileFormat {
    id: 48_105_549,
    source_type: SourceType::Wikidata,
    name: "SAS for MS-DOS Catalog",
    extensions: &["sct"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

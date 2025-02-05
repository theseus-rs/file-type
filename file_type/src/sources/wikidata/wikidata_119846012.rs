use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119846012: FileFormat = FileFormat {
    id: 119_846_012,
    source_type: SourceType::Wikidata,
    name: "Quicken Data File (Macintosh)",
    extensions: &["qdfm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

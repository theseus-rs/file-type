use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60662339: FileFormat = FileFormat {
    id: 60_662_339,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Plot Configuration File, version 1",
    extensions: &["pcp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

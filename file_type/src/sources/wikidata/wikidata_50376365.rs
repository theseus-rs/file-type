use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50376365: FileFormat = FileFormat {
    id: 50_376_365,
    source_type: SourceType::Wikidata,
    name: "File Geodatabase (Esri)",
    extensions: &["gdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

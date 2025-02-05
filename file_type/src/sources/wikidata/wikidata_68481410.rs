use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_68481410: FileFormat = FileFormat {
    id: 68_481_410,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Sheet Set file format",
    extensions: &["dst"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

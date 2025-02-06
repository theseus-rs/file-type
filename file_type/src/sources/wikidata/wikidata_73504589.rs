use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73504589: FileFormat = FileFormat {
    id: 73_504_589,
    source_type: SourceType::Wikidata,
    name: "CorelFlow",
    extensions: &["cfl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

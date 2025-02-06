use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47539298: FileFormat = FileFormat {
    id: 47_539_298,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Block Attribute Template",
    extensions: &["blk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

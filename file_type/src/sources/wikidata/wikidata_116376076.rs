use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116376076: FileFormat = FileFormat {
    id: 116_376_076,
    source_type: SourceType::Wikidata,
    name: "Access Database Addins",
    extensions: &["mda"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
